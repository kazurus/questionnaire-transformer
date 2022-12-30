use regex::Regex;
use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HtmlQuestionsProcessor {
    path: Option<PathBuf>,
}

impl HtmlQuestionsProcessor {
    pub fn parse(&self) {
        let document = fs::read_to_string(self.path.clone().unwrap().as_path()).unwrap();
        let html = Html::parse_fragment(&document);

        let questions = self.parse_questions(&html);
        println!("{:?}", questions);

        let score_values = self.parse_score(&html);
        println!("{:?}", score_values);

        let single_answers = self.parse_single_answers(&html);
        println!("{:?}", single_answers);
    }

    fn parse_score(&self, html: &Html) -> Vec<String> {
        let score_candidate_selector = Selector::parse("#pf1 .x6 div.t").unwrap();

        let score_candidates = html
            .select(&score_candidate_selector)
            .filter(|y| y.value().classes().any(|x| x == "xe"));

        let mut score_list = vec![String::from("")];

        for element in score_candidates {
            let prev_sibling = element
                .prev_sibling()
                .unwrap()
                .value()
                .as_element()
                .unwrap();

            let is_prev_current_score = prev_sibling.classes().any(|x| x == "xe");

            let current_score = element.text().collect::<Vec<_>>().join("");

            if is_prev_current_score {
                let prev_question = score_list.last_mut().unwrap();
                *prev_question = format!("{} {}", prev_question, current_score);
            } else {
                score_list.push(current_score);
            }
        }

        if score_list.get(0) == Some(&String::from("")) {
            score_list.remove(0);
        }

        let re = Regex::new(r"(\d|,){4}").unwrap();
        let score_values = score_list
            .iter()
            .map(|score| re.find(score).unwrap().as_str().to_string())
            .collect::<Vec<_>>();

        score_values
    }

    fn parse_questions(&self, html: &Html) -> Vec<String> {
        let question_candidate_selector = Selector::parse("#pf1 .x6 div.t").unwrap();

        let question_candidates = html
            .select(&question_candidate_selector)
            .filter(|y| y.value().classes().any(|i| i == "xf"));

        let mut questions = vec![String::from("")];

        for element in question_candidates {
            let prev_sibling = element
                .prev_sibling()
                .unwrap()
                .value()
                .as_element()
                .unwrap();

            let is_prev_current_question = prev_sibling.classes().any(|i| i == "xf");

            let current_question = element.text().collect::<Vec<_>>().join("");

            if is_prev_current_question {
                let prev_question = questions.last_mut().unwrap();
                *prev_question = format!("{} {}", prev_question, current_question);
            } else {
                questions.push(current_question);
            }
        }

        if questions.get(0) == Some(&String::from("")) {
            questions.remove(0);
        }

        questions
    }

    fn parse_single_answers(&self, html: &Html) -> Vec<String> {
        let answer_candidate_selector = Selector::parse("#pf1 .x6 div.t.xf+.x1").unwrap();
        let re = Regex::new(r"^[a-d].\s(?P<answer>[[\w\W]+]+)").unwrap();

        let answers = html
            .select(&answer_candidate_selector)
            .map(|element| element.text().collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .iter()
            .map(|answer| re.captures(answer).and_then(|c| c.name("answer")))
            .map(|answer| answer.unwrap().as_str().to_string())
            .collect::<Vec<_>>();

        answers
    }
}

impl From<PathBuf> for HtmlQuestionsProcessor {
    fn from(value: PathBuf) -> HtmlQuestionsProcessor {
        Self { path: Some(value) }
    }
}
