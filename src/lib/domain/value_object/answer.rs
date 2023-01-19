// #[derive(Debug, Clone)]
// #[readonly::make]
// pub struct Answer {
//     title: String,
//     kind: AnswerKind,
// }

#[derive(Debug, Clone)]
pub enum Answer {
    SingleChoice(String, bool),
    MultiChoice(String, String),
    None,
}

impl From<(String, bool)> for Answer {
    fn from(params: (String, bool)) -> Self {
        Self::SingleChoice(params.0, params.1)
    }
}

impl From<(String, String)> for Answer {
    fn from(params: (String, String)) -> Self {
        Self::MultiChoice(params.0, params.1)
    }
}

// impl Answer {
//     pub fn new(title: String, kind: AnswerKind) -> Self {
//         Self { title, kind }
//     }
// }
