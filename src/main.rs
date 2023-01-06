use std::path::PathBuf;

use questionnaire_transformer::infra::csv_questions_repository::CsvQuestionsRepository;
use questionnaire_transformer::infra::html_questions_processor::HtmlQuestionsProcessor;

fn main() {
    let html_file_path = PathBuf::from("./target/temp3.html");
    // let absolute_path = fs::canonicalize(&path.as_path()).unwrap();
    let processor = HtmlQuestionsProcessor::from(html_file_path);

    let questions = processor.parse();

    let csv_file_path = PathBuf::from("./target/temp.csv");
    let csv_repo = CsvQuestionsRepository::from(csv_file_path);
    csv_repo
        .save_all(questions)
        .expect("Can't save questions to csv file");
}
