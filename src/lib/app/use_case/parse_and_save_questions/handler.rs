use std::rc::Rc;

use crate::app::dto::dto_list::DtoList;
use crate::domain::aggregator::questions_parser::QuestionsParser;
use crate::domain::aggregator::questions_repository::QuestionsRepository;

pub struct ParseAndSaveUseCaseHandler {
    questions_repo: Rc<dyn QuestionsRepository>,
    questions_parser: Rc<dyn QuestionsParser>,
}

impl ParseAndSaveUseCaseHandler {
    pub fn new(
        questions_repo: Rc<dyn QuestionsRepository>,
        questions_parser: Rc<dyn QuestionsParser>,
    ) -> Self {
        Self {
            questions_repo,
            questions_parser,
        }
    }

    pub fn execute(&self) -> Result<DtoList<()>, Box<dyn std::error::Error>> {
        let questions = self.questions_parser.parse()?;

        self.questions_repo.save_all(&questions).map(DtoList)
    }
}
