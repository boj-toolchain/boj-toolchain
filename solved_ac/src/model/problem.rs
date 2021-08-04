use serde::{Deserialize, Serialize};

use super::{Level, Tag};

pub type ProblemId = i64;

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    #[serde(rename = "problemId")]
    pub id: ProblemId,
    pub title_ko: String,
    pub is_solvable: bool,
    pub is_partial: bool,
    pub accepted_user_count: u64,
    pub level: Level,
    pub voted_user_count: u64,
    pub is_level_locked: bool,
    pub average_tries: f64,
    pub tags: Vec<Tag>,
}
