use itertools::multizip;
use itertools::Itertools;
use regex::Regex;
use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;

use crate::domain::aggregator::questions::Questions;
use crate::domain::entity::question::Question;

#[derive(Debug)]
pub struct HtmlQuestionsProcessor {
    path: Option<PathBuf>,
}

impl HtmlQuestionsProcessor {
    pub fn parse(&self) {
        let document = fs::read_to_string(self.path.clone().unwrap().as_path()).unwrap();
        let html = Html::parse_fragment(&document);

        let questions = self.parse_questions(&html);
        let answers = self.parse_answers(&html);

        let score_tuple = self.parse_score(&html);
        let score_values = score_tuple
            .iter()
            .map(|(score, _)| score.clone())
            .collect::<Vec<_>>();
        let max_score_values = score_tuple
            .into_iter()
            .map(|(_, max)| max)
            .collect::<Vec<_>>();

        let max_questions_count = vec![questions.len(), score_values.len(), answers.len()]
            .into_iter()
            .max()
            .unwrap();

        let questions_diff = max_questions_count - questions.len();
        let score_values_diff = max_questions_count - score_values.len();
        let answers_diff = max_questions_count - answers.len();

        let questions_list = multizip((
            [questions, vec!["".to_string(); questions_diff]].concat(),
            [answers, vec![vec!["".to_string()]; answers_diff]].concat(),
            [score_values, vec!["".to_string(); score_values_diff]].concat(),
            [max_score_values, vec!["".to_string(); score_values_diff]].concat(),
        ))
        .map(|(question, answers, score, max)| Question::new(question, answers, score, max))
        .collect::<Vec<_>>();

        let res = Questions::from(questions_list);
        println!("{:?} - {:?}", res.list, res.list.len());
    }

    fn parse_score(&self, html: &Html) -> Vec<(String, String)> {
        let score_candidate_selector = Selector::parse(".s3+.s3").unwrap();
        let re = Regex::new(r"[\w\s\W]*(?P<score>[\d|,]{4})[\w\s\W]*(?P<max>[\d|,]{4})").unwrap();

        html.select(&score_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .iter()
            .map(|score_line| {
                re.captures(score_line.as_str())
                    .map(|cap| (cap.name("score"), cap.name("max")))
            })
            .map(|mayby_score| mayby_score.expect("Can't parse score tuple"))
            .map(|(one, two)| {
                (
                    one.expect("Can't parse score").as_str().to_string(),
                    two.expect("Can't parse max score").as_str().to_string(),
                )
            })
            .collect::<Vec<_>>()
    }

    fn parse_questions(&self, html: &Html) -> Vec<String> {
        let question_candidate_selector =
            Selector::parse(r#".s3+p+p+p[style*="padding-top: 5pt;padding-left: 19pt;"]"#).unwrap();

        let questions = html
            .select(&question_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .collect();

        questions
    }

    fn parse_answers(&self, html: &Html) -> Vec<Vec<String>> {
        let answer_candidate_selector = Selector::parse(".s4+table+span.p").unwrap();
        let re = Regex::new(r"^[\w].\s(?P<answer>[[\w\W]+]+)").unwrap();

        let answers = html
            .select(&answer_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .group_by(|elem| elem.starts_with("a. "))
            .into_iter()
            .map(|(_, group)| group.collect::<Vec<_>>())
            .chunks(2)
            .into_iter()
            .map(|chunks| chunks.flatten().collect::<Vec<_>>())
            .map(|answers| {
                answers
                    .iter()
                    .map(|answer| re.captures(answer.as_str()).and_then(|c| c.name("answer")))
                    .map(|answer| answer.unwrap().as_str().to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        answers
    }
}

impl From<PathBuf> for HtmlQuestionsProcessor {
    fn from(value: PathBuf) -> HtmlQuestionsProcessor {
        Self { path: Some(value) }
    }
}
