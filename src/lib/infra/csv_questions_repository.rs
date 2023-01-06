use std::path::PathBuf;

use crate::domain::aggregator::questions::Questions;
// use crate::domain::entity::question::Question;

#[derive(Debug)]
pub struct CsvQuestionsRepository {
    path: PathBuf,
}

impl CsvQuestionsRepository {
    pub fn save_all(&self, questions: Questions) -> Result<(), &str> {
        let mut wtr = csv::Writer::from_path(&self.path).expect("Can't open csv file");

        questions.list.iter().for_each(|question_item| {
            // let Question { question, answers, score, max_score } = question_item;
            let answers_string = question_item
                .answers
                .iter()
                .map(|(answer, status)| format!("{answer} - {status}"))
                .collect::<Vec<_>>()
                .join("\n");

            wtr.write_record(&[
                question_item.question.clone(),
                answers_string,
                question_item.score.clone(),
                question_item.max_score.clone(),
            ])
            .expect("Can't save question to csv")
        });

        Ok(())
    }
}

impl From<PathBuf> for CsvQuestionsRepository {
    fn from(path: PathBuf) -> CsvQuestionsRepository {
        Self { path }
    }
}
