use std::path::PathBuf;
use std::rc::Rc;

use questionnaire_transformer::infra::csv_questions_repository::CsvQuestionsRepository;
// use questionnaire_transformer::infra::html_questions_parser::HtmlQuestionsParser;
use questionnaire_transformer::infra::html_questions_parser_v2::HtmlQuestionsParserV2;

use questionnaire_transformer::app::use_case::{
    get_all_questions::handler::GetAllQuestionsUseCaseHandler,
    parse_and_save_questions::handler::ParseAndSaveUseCaseHandler,
};

fn main() {
    let html_file_path = PathBuf::from("./target/temp-html-1.html");
    // let absolute_path = fs::canonicalize(&path.as_path()).unwrap();
    let parser = Rc::new(HtmlQuestionsParserV2::from(html_file_path));

    let csv_file_path = PathBuf::from("./target/temp.csv");
    let csv_repo = Rc::new(CsvQuestionsRepository::from(csv_file_path));

    ParseAndSaveUseCaseHandler::new(csv_repo.clone(), parser)
        .execute()
        .unwrap();

    let questions_from_csv = GetAllQuestionsUseCaseHandler::new(csv_repo)
        .execute()
        .unwrap();
    println!("{:?}", questions_from_csv);
}
