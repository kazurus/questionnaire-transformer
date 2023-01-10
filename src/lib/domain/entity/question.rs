use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
#[readonly::make]
pub struct Question {
    pub partial_hash: u64,
    pub strict_hash: u64,
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
        let partial_hash = Question::calculate_partial_hash(&question, &max_score, &answers);
        let strict_hash = Question::calculate_strict_hash(&partial_hash, &score, &answers);
        // println!("a - {}, p - {}, h - {}", &question.chars().take(5).collect::<String>(), partial_hash, strict_hash);

        Self {
            partial_hash,
            strict_hash,
            question,
            score,
            max_score,
            answers,
        }
    }

    pub fn has_max_score(&self) -> bool {
        self.score == self.max_score
    }

    pub fn calculate_strict_hash(
        partial_hash: &u64,
        score: &str,
        answers: &[(String, bool)],
    ) -> u64 {
        let mut state = DefaultHasher::new();

        partial_hash.to_string().hash(&mut state);
        score.trim().hash(&mut state);

        let vec = answers
            .iter()
            .sorted()
            .map(|(_, state)| state.to_string().trim().to_string())
            .collect::<Vec<_>>();
        vec.join("-").hash(&mut state);

        state.finish()
    }

    pub fn calculate_partial_hash(
        question: &str,
        max_score: &str,
        answers: &[(String, bool)],
    ) -> u64 {
        let mut state = DefaultHasher::new();

        question.trim().hash(&mut state);
        max_score.trim().hash(&mut state);

        let vec = answers
            .iter()
            .sorted()
            .map(|(a, _)| a.trim())
            .collect::<Vec<_>>();
        vec.join("-").hash(&mut state);

        state.finish()
    }
}

impl Hash for Question {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.strict_hash.to_string().hash(state);
    }
}
