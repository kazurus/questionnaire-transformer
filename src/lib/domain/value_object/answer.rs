#[derive(Debug, Clone)]
pub enum Answer {
    Choice(String, bool),
    None,
}

impl From<(String, bool)> for Answer {
    fn from(params: (String, bool)) -> Self {
        Self::Choice(params.0, params.1)
    }
}
