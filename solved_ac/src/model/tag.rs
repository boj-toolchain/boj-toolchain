use serde::{Deserialize, Serialize};

use super::DisplayName;

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    key: String,
    is_meta: bool,
    boj_tag_id: i64,
    problem_count: u64,
    display_names: Vec<DisplayName>,
}
