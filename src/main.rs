use questionnaire_transformer::infra::pdf_questions_processor::PdfQuestionsProcessor;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./target/temp2.pdf");
    // let absolute_path = fs::canonicalize(&path.as_path()).unwrap();

    let processor = PdfQuestionsProcessor::from(path);
    processor.parse();
}
