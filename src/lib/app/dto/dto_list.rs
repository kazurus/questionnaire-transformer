#[derive(Debug)]
pub struct DtoList<T>(pub Vec<T>);

impl<T> From<Vec<T>> for DtoList<T> {
    fn from(vec: Vec<T>) -> DtoList<T> {
        Self(vec)
    }
}
