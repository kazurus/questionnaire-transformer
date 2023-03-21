use clap::Parser;

use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::infra::csv_questions_repository::CsvQuestionsRepository;
// use crate::infra::html_questions_parser::HtmlQuestionsParser;
use crate::infra::html_questions_parser_v2::HtmlQuestionsParserV2;

use crate::app::use_case::{
    get_all_questions::handler::GetAllQuestionsUseCaseHandler,
    parse_merge_and_save_questions::handler::ParseMergeAndSaveUseCaseHandler,
};

use std::fs;

#[derive(Parser, Debug)]
pub struct ConsoleArgsApi {
    // name: String,
    // #[arg(short, long)]
    // no_warnings: bool,
    #[arg(short = 'i', long = "input")]
    input: PathBuf,
    #[arg(short = 'o', long = "output")]
    output: PathBuf,
}

impl ConsoleArgsApi {
    pub fn start() -> Result<(), Box<dyn std::error::Error>> {
        let args = Self::parse();

        if !args.output.is_file() {
            return Err("--output should be a file".into());
        }

        let is_dir = args.input.is_dir();
        let is_file = args.input.is_file();
        match (is_dir, is_file) {
            (true, false) => ConsoleArgsApi::parse_folder(&args.input, &args.output)?,
            (false, true) => ConsoleArgsApi::parse_file(&args.input, &args.output)?,
            _ => return Err("--input should be a file or folder".into()),
        }

        Ok(())
    }

    fn parse_file(file: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let parser = Rc::new(HtmlQuestionsParserV2::from(file.to_path_buf()));
        let csv_repo = Rc::new(CsvQuestionsRepository::from(output.to_path_buf()));

        ParseMergeAndSaveUseCaseHandler::new(csv_repo.clone(), parser).execute()?;

        let questions_from_csv = GetAllQuestionsUseCaseHandler::new(csv_repo).execute()?;
        println!("{:?}", questions_from_csv);

        Ok(())
    }

    fn parse_folder(folder: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::read_dir(folder)?.try_for_each(|mayby_file| -> Result<(), Box<dyn std::error::Error>> {
            let file = mayby_file?;
            ConsoleArgsApi::parse_file(&file.path(), output)?;

            Ok(())
        })
    }
}
