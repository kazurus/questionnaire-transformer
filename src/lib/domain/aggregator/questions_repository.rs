use crate::domain::aggregator::questions::Questions;

pub trait QuestionsRepository {
    fn get_all(&self) -> Result<Questions, Box<dyn std::error::Error>>;
    fn get_all_soft(&self) -> Result<Questions, Box<dyn std::error::Error>>;
    fn save_all(&self, questions: &Questions) -> Result<Vec<()>, Box<dyn std::error::Error>>;
}
