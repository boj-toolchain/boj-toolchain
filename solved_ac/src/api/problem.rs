use serde::Serialize;
use surf::StatusCode;

use crate::{
    api::BASE_URL,
    model::{Problem, ProblemId},
};

use super::SolvedAcError;

#[derive(Serialize)]
struct GetByIdQuery {
    #[serde(rename = "problemId")]
    id: ProblemId,
}

pub async fn get_by_id(id: ProblemId) -> Result<Option<Problem>, SolvedAcError> {
    let mut res = surf::get(format!("{BASE_URL}/problem/show", BASE_URL = BASE_URL))
        .query(&GetByIdQuery { id })?
        .await?;
    if res.status() == StatusCode::NotFound {
        Ok(None)
    } else {
        Ok(Some(res.body_json().await?))
    }
}
