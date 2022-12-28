use scraper::{Html, Selector};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HtmlQuestionsProcessor {
    path: Option<PathBuf>,
}

impl HtmlQuestionsProcessor {
    pub fn parse(&self) {
        let page = fs::read_to_string(self.path.clone().unwrap().as_path()).unwrap();
        let html = Html::parse_fragment(&page);
        let selector = Selector::parse("#pf1 .x6 div.t.ws8").unwrap();
        for element in html.select(&selector) {
            println!("{:?}", element.text().next().unwrap());
        }
    }
}

impl From<PathBuf> for HtmlQuestionsProcessor {
    fn from(value: PathBuf) -> HtmlQuestionsProcessor {
        Self { path: Some(value) }
    }
}
