use std::collections::hash_map::HashMap;

use crate::domain::entity::question::Question;

#[derive(Debug, Clone)]
pub struct Questions {
    pub list: Vec<Question>,
}

impl Questions {
    pub fn new(list: Vec<Question>) -> Self {
        Self { list }
    }

    pub fn concat(questions_first: &Self, questions_second: &Self) -> Self {
        let questions_list = [questions_first.list.clone(), questions_second.list.clone()].concat();

        Questions::from(questions_list)
    }

    pub fn dedup(questions: &Self) -> Self {
        questions
            .list
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, question| {
                let hash = question.calculate_hash();
                let q = acc.entry(hash).or_insert_with(|| question.clone());

                if !q.has_max_score() {
                    acc.insert(hash, question);
                }

                acc
            })
            .values()
            .cloned()
            .collect::<Vec<Question>>()
            .into()
    }
}

impl From<Vec<Question>> for Questions {
    fn from(questions: Vec<Question>) -> Self {
        Self { list: questions }
    }
}
