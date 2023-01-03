#[allow(dead_code)]
#[derive(Debug)]
pub struct Question {
    question: String,
    answers: Vec<String>,
    score: String,
    max_score: String,
}

impl Question {
    pub fn new(question: String, answers: Vec<String>, score: String, max_score: String) -> Self {
        Self {
            question,
            answers,
            score,
            max_score,
        }
    }
}
