use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[repr(i64)]
pub enum Language {
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "en")]
    English,
}
