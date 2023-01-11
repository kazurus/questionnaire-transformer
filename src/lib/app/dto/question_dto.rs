use std::ops::Deref;

use crate::domain::entity::question::{Question, ReadOnlyQuestion};

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

        Self {
            question: question.into(),
            score: score.into(),
            max_score: max_score.into(),
            answers: answers.clone(),
        }
    }
}
