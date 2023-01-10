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
                let questions_map: &mut HashMap<u64, Question> =
                    acc.entry(question.partial_hash).or_default();

                match question.has_max_score() {
                    true => questions_map.entry(u64::MAX).or_insert(question),
                    _ => questions_map
                        .entry(question.strict_hash)
                        .or_insert(question),
                };

                acc
            })
            .values()
            .fold(vec![] as Vec<Question>, |mut acc, q_map| {
                let questions_vec: Vec<Question> = match q_map.get(&u64::MAX) {
                    Some(question) => vec![question.clone()],
                    _ => q_map.values().cloned().collect(),
                };

                acc.extend(questions_vec);

                acc
            })
            .into()
    }
}

impl From<Vec<Question>> for Questions {
    fn from(questions: Vec<Question>) -> Self {
        Self { list: questions }
    }
}
