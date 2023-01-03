#[allow(dead_code)]
#[derive(Debug)]
pub struct Question {
    question: String,
    score: String,
    answers: Vec<String>,
}

impl Question {
    pub fn new(question: String, score: String, answers: Vec<String>) -> Self {
        Self {
            question,
            score,
            answers,
        }
    }
}
