use std::rc::Rc;

use crate::domain::aggregator::questions::Questions;
use crate::domain::aggregator::questions_parser::QuestionsParser;
use crate::domain::aggregator::questions_repository::QuestionsRepository;

pub struct ParseMergeAndSaveUseCaseHandler {
    questions_repo: Rc<dyn QuestionsRepository>,
    questions_parser: Rc<dyn QuestionsParser>,
}

impl ParseMergeAndSaveUseCaseHandler {
    pub fn new(
        questions_repo: Rc<dyn QuestionsRepository>,
        questions_parser: Rc<dyn QuestionsParser>,
    ) -> Self {
        Self {
            questions_repo,
            questions_parser,
        }
    }

    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let questions_from_csv = self.questions_repo.get_all_soft()?;
        let questions_from_parsing = self.questions_parser.parse()?;

        let questions_with_dup = Questions::concat(&questions_from_csv, &questions_from_parsing);
        let questions = Questions::dedup(&questions_with_dup);

        self.questions_repo.save_all(&questions)?;

        Ok(())
    }
}
