#[allow(dead_code)]
#[derive(Debug)]
pub struct Question {
    question: String,
    answers: Vec<(String, bool)>,
    score: String,
    max_score: String,
}

impl Question {
    pub fn new(
        question: String,
        score: String,
        max_score: String,
        answers: Vec<(String, bool)>,
    ) -> Self {
        Self {
            question,
            score,
            max_score,
            answers,
        }
    }
}
