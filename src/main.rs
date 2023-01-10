use std::path::PathBuf;

use questionnaire_transformer::infra::csv_questions_repository::CsvQuestionsRepository;
use questionnaire_transformer::infra::html_questions_processor::HtmlQuestionsProcessor;

use questionnaire_transformer::domain::aggregator::questions::Questions;

fn main() {
    let html_file_path = PathBuf::from("./target/temp3.html");
    // let absolute_path = fs::canonicalize(&path.as_path()).unwrap();
    let processor = HtmlQuestionsProcessor::from(html_file_path);

    let questions = processor.parse();

    let csv_file_path = PathBuf::from("./target/temp.csv");
    let csv_repo = CsvQuestionsRepository::from(csv_file_path);
    csv_repo
        .save_all(&questions)
        .expect("Can't save questions to csv file");

    let questions_from_csv = csv_repo
        // .get_all()
        .get_all_soft()
        .expect("Can't get questions from csv file");
    // println!("{:?}", questions_from_csv);

    let questions_with_duplicates = Questions::concat(&questions_from_csv, &questions_from_csv);
    println!(
        "{:?}",
        // "{:?} - {}",
        // questions_with_duplicates,
        questions_with_duplicates.list.len()
    );
    // let d = questions_with_duplicates.list.iter().inspect(|q| {
    //     println!("dup a - {}, p - {}, h - {}", q.question.chars().take(10).collect::<String>(), q.partial_hash, q.strict_hash);
    // }).collect::<Vec<_>>();

    let questions_without_duplicates = Questions::dedup(&questions_with_duplicates);
    println!(
        "{:?}",
        // "{:?} - {}",
        // questions_without_duplicates,
        questions_without_duplicates.list.len()
    );

    let csv_file_path_dedup = PathBuf::from("./target/temp1.csv");
    let csv_repo_dedup = CsvQuestionsRepository::from(csv_file_path_dedup);
    csv_repo_dedup
        .save_all(&questions_without_duplicates)
        .expect("Can't save questions to csv file");
}
