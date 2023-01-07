use std::path::PathBuf;

use itertools::Itertools;

use crate::domain::aggregator::questions::Questions;
use crate::domain::entity::question::Question;

#[derive(Debug)]
pub struct CsvQuestionsRepository {
    path: PathBuf,
}

impl CsvQuestionsRepository {
    pub fn save_all(&self, questions: Questions) -> Result<(), &str> {
        let mut wtr = csv::Writer::from_path(&self.path).expect("Can't open csv file for writing");

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

    pub fn get_all(&self) -> Result<Questions, &str> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&self.path)
            .expect("Can't open csv file for reading");

        let questions_vec = rdr
            .records()
            .map(|maybe_rec| maybe_rec.expect("Can't read csv string"))
            .map(|record| {
                let pos = record
                    .position()
                    .expect("Can't get rec position from csv")
                    .line();
                let msg = format!("Can't csv parse on {} line", pos);

                let question = record.get(0).expect(&msg).to_string();
                let score = record.get(2).expect(&msg).to_string();
                let max_score = record.get(3).expect(&msg).to_string();
                let answers = record
                    .get(1)
                    .expect(&msg)
                    .split('\n')
                    .map(|line| {
                        line.split(" - ")
                            .collect_tuple::<(&str, &str)>()
                            .expect("Can't collect answer tuple")
                    })
                    .map(|(answer, status)| match status {
                        "true" => (answer.to_string(), true),
                        _ => (answer.to_string(), false),
                    })
                    .collect::<Vec<_>>();

                Question::new(question, score, max_score, answers)
            })
            .collect::<Vec<_>>();

        Ok(Questions::from(questions_vec))
    }
}

impl From<PathBuf> for CsvQuestionsRepository {
    fn from(path: PathBuf) -> CsvQuestionsRepository {
        Self { path }
    }
}
