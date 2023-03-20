use regex::Regex;
use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;

use crate::domain::aggregator::questions::Questions;
use crate::domain::aggregator::questions_parser::QuestionsParser;
use crate::domain::entity::question::Question;
use crate::domain::value_object::answer::Answer;

#[derive(Debug)]
pub struct HtmlQuestionsParserV2 {
    path: PathBuf,
}

impl QuestionsParser for HtmlQuestionsParserV2 {
    fn parse(&self) -> Result<Questions, Box<dyn std::error::Error>> {
        let document = fs::read_to_string(self.path.clone().as_path()).unwrap();
        let html = Html::parse_fragment(&document);

        let questions_list_v2 = self
            .parse_questions_blocks(&html)
            .iter()
            .map(|question_block| {
                let question = self.parse_question(question_block);
                let (score, max_score) = self.parse_score(question_block);
                let answers = self.parse_answers(question_block);

                Question::new(question, score, max_score, answers)
            })
            .collect::<Vec<_>>();

        Ok(Questions::from(questions_list_v2))
    }
}

impl HtmlQuestionsParserV2 {
    fn parse_questions_blocks(&self, html: &Html) -> Vec<Html> {
        let items_selector = Selector::parse(".multichoice").unwrap();

        html.select(&items_selector)
            .map(|elem| Html::parse_fragment(&elem.html()))
            .collect::<Vec<_>>()
    }

    fn parse_question(&self, question_block: &Html) -> String {
        let question_selector = Selector::parse(".content .qtext").unwrap();

        question_block
            .select(&question_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .next()
            .unwrap_or_default()
    }

    fn parse_score(&self, question_block: &Html) -> (String, String) {
        let score_candidate_selector = Selector::parse(".info .grade").unwrap();
        let re = Regex::new(r"[\w\s\W]*(?P<score>[\d|,]{4})[\w\s\W]*(?P<max>[\d|,]{4})").unwrap();

        question_block
            .select(&score_candidate_selector)
            .map(|elem| elem.text().collect::<Vec<_>>().join(""))
            .next()
            .map(|score_line| {
                re.captures(score_line.as_str())
                    .map(|cap| (cap.name("score"), cap.name("max")))
                    .map(|(score_match, max_score_match)| {
                        (
                            score_match
                                .map(|score| score.as_str().to_string())
                                .unwrap_or_default(),
                            max_score_match
                                .map(|score| score.as_str().to_string())
                                .unwrap_or_default(),
                        )
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }

    fn parse_answers(&self, question_block: &Html) -> Vec<Answer> {
        let answer_candidate_selector = Selector::parse(".answer > div").unwrap();
        let answer_status_candidate_selector = Selector::parse("input").unwrap();
        let re = Regex::new(r"^[\w].\s(?P<answer>[[\w\W]+]+)").unwrap();

        question_block
            .select(&answer_candidate_selector)
            .map(|elem| {
                let input_status = elem
                    .select(&answer_status_candidate_selector)
                    .next()
                    .map(|html_input| html_input.value().attr("checked"))
                    .map(|input_check_attr| match input_check_attr {
                        Some(val) => val == "checked",
                        None => false,
                    })
                    .unwrap_or_default();

                (elem.text().collect::<Vec<_>>().join(""), input_status)
            })
            .map(|(elem, status)| {
                let text_without_prefix = re
                    .captures(elem.as_str())
                    .and_then(|c| c.name("answer"))
                    .map(|text_match| text_match.as_str().to_string())
                    .unwrap_or_default();

                (text_without_prefix, status)
            })
            .map(|(elem, status)| Answer::Choice(elem, status))
            .collect::<Vec<_>>()
    }
}

impl From<PathBuf> for HtmlQuestionsParserV2 {
    fn from(path: PathBuf) -> HtmlQuestionsParserV2 {
        Self { path }
    }
}
