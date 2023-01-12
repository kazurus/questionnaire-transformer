use std::rc::Rc;

use crate::app::dto::{dto_list::DtoList, question_dto::QuestionDto};
use crate::domain::aggregator::questions_repository::QuestionsRepository;

pub struct GetAllQuestionsUseCaseHandler {
    questions_repo: Rc<dyn QuestionsRepository>,
}

impl GetAllQuestionsUseCaseHandler {
    pub fn new(questions_repo: Rc<dyn QuestionsRepository>) -> Self {
        Self { questions_repo }
    }

    pub fn execute(&self) -> Result<DtoList<QuestionDto>, Box<dyn std::error::Error>> {
        self.questions_repo
            // .get_all()
            .get_all_soft()
            .map(|questions_vec| {
                questions_vec
                    .list
                    .iter()
                    .map(QuestionDto::from)
                    .collect::<Vec<QuestionDto>>()
            })
            .map(DtoList)
    }
}
