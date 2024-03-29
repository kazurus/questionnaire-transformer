use std::ops::Deref;
use std::path::PathBuf;

use csv::StringRecord;
use itertools::Itertools;

use crate::domain::aggregator::{questions::Questions, questions_repository::QuestionsRepository};
use crate::domain::entity::question::{Question, ReadOnlyQuestion};
use crate::domain::value_object::answer::Answer;

#[derive(Debug)]
pub struct CsvQuestionsRepository {
    path: PathBuf,
}

struct QuestionFields(Option<String>, Option<String>, Option<String>, Vec<Answer>);

impl QuestionsRepository for CsvQuestionsRepository {
    fn save_all(&self, questions: &Questions) -> Result<Vec<()>, Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(&self.path)?;
        wtr.write_record(["Question", "Answers", "Score", "Max Score"])?;

        questions
            .list
            .iter()
            .map(|question_item| {
                let ReadOnlyQuestion {
                    question,
                    answers,
                    score,
                    max_score,
                    ..
                } = question_item.deref();

                let answers_string = answers
                    .clone()
                    .into_iter()
                    .map(|answer| match answer {
                        Answer::Choice(title, is_checked) => {
                            format!("{title} -> {is_checked}")
                        }
                        _ => String::default(),
                    })
                    .join("\n");

                wtr.write_record(&[
                    question.clone(),
                    answers_string,
                    score.clone(),
                    max_score.clone(),
                ])
            })
            .map(|r| r.map_err(|e| e.into()))
            .collect::<Result<Vec<_>, _>>()
    }

    fn get_all(&self) -> Result<Questions, Box<dyn std::error::Error>> {
        let questions_vec = self
            .parse_records_to_questions_fields()
            .into_iter()
            .map(|QuestionFields(question, score, max_score, answers)| {
                Some(Question::new(question?, score?, max_score?, answers))
            })
            .enumerate()
            .map(|(line_id, mayby_question)| {
                mayby_question.ok_or(format!("Can't csv parse on {} line", line_id + 1))
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(Questions::from(questions_vec?))
    }

    fn get_all_soft(&self) -> Result<Questions, Box<dyn std::error::Error>> {
        let questions_vec = self
            .parse_records_to_questions_fields()
            .into_iter()
            .map(|QuestionFields(question, score, max_score, answers)| {
                Some(Question::new(
                    question.unwrap_or_default(),
                    score.unwrap_or_default(),
                    max_score.unwrap_or_default(),
                    answers,
                ))
            })
            .enumerate()
            .map(|(line_id, mayby_question)| {
                mayby_question.ok_or(format!("Can't csv parse on {} line", line_id + 1))
            })
            .collect::<Result<Vec<_>, _>>();

        Ok(Questions::from(questions_vec?))
    }
}

impl CsvQuestionsRepository {
    fn parse_records_to_questions_fields(&self) -> Vec<QuestionFields> {
        csv::ReaderBuilder::new()
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
        let answers = self.parse_record_answers(record.get(1).unwrap_or_default());

        QuestionFields(question, score, max_score, answers)
    }

    fn parse_record_answers(&self, answers: &str) -> Vec<Answer> {
        answers
            .split('\n')
            .map(|line| line.split(" -> ").collect_tuple::<(&str, &str)>())
            .map(|res| match res {
                Some((answer, "true")) => Answer::Choice(answer.to_string(), true),
                Some((answer, "false")) => Answer::Choice(answer.to_string(), false),
                _ => Answer::None,
            })
            .collect::<Vec<_>>()
    }
}

impl From<PathBuf> for CsvQuestionsRepository {
    fn from(path: PathBuf) -> CsvQuestionsRepository {
        Self { path }
    }
}
