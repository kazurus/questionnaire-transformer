use std::ops::Deref;

use crate::domain::{
    entity::question::{Question, ReadOnlyQuestion},
    value_object::answer::Answer,
};

#[readonly::make]
#[derive(Debug, Clone)]
pub struct QuestionDto {
    pub question: String,
    pub score: String,
    pub max_score: String,
    pub answers: Vec<(String, bool)>,
}

impl From<&Question> for QuestionDto {
    fn from(question: &Question) -> Self {
        let ReadOnlyQuestion {
            question,
            score,
            max_score,
            answers,
            ..
        } = question.deref();

        let answers_tuple = answers
            .clone()
            .into_iter()
            .map(|answer| match answer {
                Answer::Choice(title, is_checked) => (title, is_checked),
                _ => (String::default(), bool::default()),
            })
            .collect::<Vec<_>>();

        Self {
            question: question.into(),
            score: score.into(),
            max_score: max_score.into(),
            answers: answers_tuple,
        }
    }
}
