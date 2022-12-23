use pdf::file::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PdfQuestionsProcessor {
    path: Option<PathBuf>,
}

impl PdfQuestionsProcessor {
    pub fn parse(&self) {
        let pdf_page = File::open(self.path.clone().unwrap().as_path())
            .unwrap()
            .get_page(0)
            .unwrap();
        println!("try contents {:?}", &pdf_page.contents);
    }
}

impl From<PathBuf> for PdfQuestionsProcessor {
    fn from(value: PathBuf) -> PdfQuestionsProcessor {
        Self { path: Some(value) }
    }
}
