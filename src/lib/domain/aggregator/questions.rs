use crate::domain::entity::question::Question;

#[derive(Debug)]
pub struct Questions {
    pub list: Vec<Question>,
}

impl Questions {
    pub fn new(list: Vec<Question>) -> Self {
        Self { list }
    }
}

impl From<Vec<Question>> for Questions {
    fn from(questions: Vec<Question>) -> Self {
        Self { list: questions }
    }
}
