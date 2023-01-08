use std::path::PathBuf;

use csv::StringRecord;
use itertools::Itertools;

use crate::domain::aggregator::questions::Questions;
use crate::domain::entity::question::Question;

#[derive(Debug)]
pub struct CsvQuestionsRepository {
    path: PathBuf,
}

struct QuestionFields(
    Option<String>,
    Option<String>,
    Option<String>,
    Vec<Option<(String, bool)>>,
);

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

    pub fn get_all(&self) -> Result<Questions, Box<dyn std::error::Error>> {
        let questions_vec = self
            .parse_records_to_questions_fields()
            .into_iter()
            .map(|QuestionFields(question, score, max_score, answers)| {
                let answers = answers.into_iter().collect::<Option<Vec<_>>>();

                Some(Question::new(question?, score?, max_score?, answers?))
            })
            .enumerate()
            .map(|(line_id, mayby_question)| {
                mayby_question.ok_or(format!("Can't csv parse on {} line", line_id + 1))
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(Questions::from(questions_vec?))
    }

    pub fn get_all_soft(&self) -> Result<Questions, Box<dyn std::error::Error>> {
        let questions_vec = self
            .parse_records_to_questions_fields()
            .into_iter()
            .map(|QuestionFields(question, score, max_score, answers)| {
                let answers = answers
                    .into_iter()
                    .filter(Option::is_some)
                    .collect::<Option<Vec<_>>>();

                Some(Question::new(
                    question.unwrap_or_default(),
                    score.unwrap_or_default(),
                    max_score.unwrap_or_default(),
                    answers.unwrap_or_default(),
                ))
            })
            .enumerate()
            .map(|(line_id, mayby_question)| {
                mayby_question.ok_or(format!("Can't csv parse on {} line", line_id + 1))
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(Questions::from(questions_vec?))
    }

    fn parse_records_to_questions_fields(&self) -> Vec<QuestionFields> {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&self.path)
            .map_err(|e| format!("Can't open csv file for reading: {e}"))
            .map(|r| r.into_records())
            .into_iter()
            .flatten()
            .map(|record| self.map_record_to_question_fields(record.unwrap_or_default()))
            .collect::<Vec<_>>()
    }

    fn map_record_to_question_fields(&self, record: StringRecord) -> QuestionFields {
        let question = record.get(0).map(|q| q.to_string());
        let score = record.get(2).map(|q| q.to_string());
        let max_score = record.get(3).map(|q| q.to_string());
        let answers = self
            .parse_record_answers(record.get(1).unwrap_or_default())
            .into_iter()
            .collect::<Vec<Option<_>>>();

        QuestionFields(question, score, max_score, answers)
    }

    fn parse_record_answers(&self, answers: &str) -> Vec<Option<(String, bool)>> {
        answers
            .split('\n')
            .map(|line| line.split(" - ").collect_tuple::<(&str, &str)>())
            .map(|res| {
                let (answer, status) = res?;

                match status {
                    "true" => Some((answer.to_string(), true)),
                    _ => Some((answer.to_string(), false)),
                }
            })
            .collect::<Vec<Option<_>>>()
    }
}

impl From<PathBuf> for CsvQuestionsRepository {
    fn from(path: PathBuf) -> CsvQuestionsRepository {
        Self { path }
    }
}
