use crate::domain::aggregator::questions::Questions;

pub trait QuestionsParser {
    fn parse(&self) -> Result<Questions, Box<dyn std::error::Error>>;
}
