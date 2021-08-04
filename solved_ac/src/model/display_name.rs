use serde::{Deserialize, Serialize};

use super::Language;

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct DisplayName {
    language: Language,
    name: String,
    short: String,
}
