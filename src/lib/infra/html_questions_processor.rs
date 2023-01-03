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
        let score_values = self.parse_score(&html);
        let answers = self.parse_single_answers(&html);

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
        ))
        .map(|(question, answers, score)| Question::new(question, score, answers))
        .collect::<Vec<_>>();

        let res = Questions::from(questions_list);
        println!("{:?} - {:?}", res.list, res.list.len());
    }

    fn parse_score(&self, html: &Html) -> Vec<String> {
        let score_candidate_selector = Selector::parse(".s3+.s3").unwrap();

        let score_list = html
            .select(&score_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""));

        let re = Regex::new(r"(\d|,){4}").unwrap();

        score_list
            .map(|score| re.find(score.as_str()).unwrap().as_str().to_string())
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

    fn parse_single_answers(&self, html: &Html) -> Vec<Vec<String>> {
        let answer_candidate_selector = Selector::parse(".s4+table+span.p").unwrap();
        let re = Regex::new(r"^[\w].\s(?P<answer>[[\w\W]+]+)").unwrap();

        let answers = html
            .select(&answer_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .group_by(|elem| elem.starts_with("a. "))
            .into_iter()
            .map(|(_, group)| group.collect_vec())
            .chunks(2)
            .into_iter()
            .map(|chunks| chunks.flatten().collect_vec())
            .map(|answers| {
                answers
                    .iter()
                    .map(|answer| re.captures(answer.as_str()).and_then(|c| c.name("answer")))
                    .map(|answer| answer.unwrap().as_str().to_string())
                    .collect_vec()
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
