#[derive(Debug)]
pub enum StoreErrorType {
    DocumentNotFound,
    DocumentAlreadyExists,
}

pub type StoreResult<T> = Result<T, StoreErrorType>;