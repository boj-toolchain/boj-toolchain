use thiserror::Error;

pub mod problem;

pub const BASE_URL: &str = "https://solved.ac/api/v3";

#[derive(Error, Debug)]
pub enum SolvedAcError {
    #[error("{0}")]
    Surf(surf::Error),
}

impl From<surf::Error> for SolvedAcError {
    fn from(e: surf::Error) -> Self {
        SolvedAcError::Surf(e)
    }
}
