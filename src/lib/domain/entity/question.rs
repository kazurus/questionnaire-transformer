#[derive(Debug)]
#[readonly::make]
pub struct Question {
    pub question: String,
    pub answers: Vec<(String, bool)>,
    pub score: String,
    pub max_score: String,
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
