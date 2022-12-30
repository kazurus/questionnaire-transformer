#[allow(dead_code)]
#[derive(Debug)]
pub struct Question {
    question: String,
    score: String,
    answer: String,
}

impl Question {
    pub fn new(question: String, score: String, answer: String) -> Self {
        Self {
            question,
            score,
            answer,
        }
    }
}
