use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HtmlQuestionsProcessor {
    path: Option<PathBuf>,
}

impl HtmlQuestionsProcessor {
    pub fn parse(&self) {
        self.parse_questions()
    }

    fn parse_questions(&self) {
        let page = fs::read_to_string(self.path.clone().unwrap().as_path()).unwrap();
        let html = Html::parse_fragment(&page);

        let question_candidate_selector = Selector::parse("#pf1 .x6 div.t").unwrap();

        let question_candidates = html.select(&question_candidate_selector).filter(|y| {
            y.value()
                .classes()
                .collect::<Vec<_>>()
                .iter()
                .any(|&i| i == "xf")
        });

        let mut questions = vec![String::from("")];

        for element in question_candidates {
            let prev_sibling = element
                .prev_sibling()
                .unwrap()
                .value()
                .as_element()
                .unwrap();

            let is_prev_current_question = prev_sibling
                .classes()
                .collect::<Vec<_>>()
                .iter()
                .any(|&i| i == "xf");

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

        println!("{:?}", questions);
    }
}

impl From<PathBuf> for HtmlQuestionsProcessor {
    fn from(value: PathBuf) -> HtmlQuestionsProcessor {
        Self { path: Some(value) }
    }
}
