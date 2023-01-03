use questionnaire_transformer::infra::html_questions_processor::HtmlQuestionsProcessor;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./target/temp3.html");
    // let absolute_path = fs::canonicalize(&path.as_path()).unwrap();

    let processor = HtmlQuestionsProcessor::from(path);
    processor.parse();
}
