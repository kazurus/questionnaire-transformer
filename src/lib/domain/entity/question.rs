use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone)]
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

    pub fn has_max_score(&self) -> bool {
        self.score == self.max_score
    }

    pub fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Hash for Question {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.question.trim().hash(state);
        self.max_score.trim().hash(state);

        let mut vec = self
            .answers
            .iter()
            .map(|(a, _)| a.trim())
            .collect::<Vec<_>>();
        vec.sort();
        vec.join("-").hash(state);
    }
}
