use serde_repr::{Deserialize_repr, Serialize_repr};

///
/// Represents a level of problem.
///
#[derive(Serialize_repr, Deserialize_repr, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(untagged)]
#[repr(i64)]
pub enum Level {
    Unrated = 0,
    Bronze5 = 1,
    Bronze4 = 2,
    Bronze3 = 3,
    Bronze2 = 4,
    Bronze1 = 5,
    Silver5 = 6,
    Silver4 = 7,
    Silver3 = 8,
    Silver2 = 9,
    Silver1 = 10,
    Gold5 = 11,
    Gold4 = 12,
    Gold3 = 13,
    Gold2 = 14,
    Gold1 = 15,
    Platinum5 = 16,
    Platinum4 = 17,
    Platinum3 = 18,
    Platinum2 = 19,
    Platinum1 = 20,
    Diamond5 = 21,
    Diamond4 = 22,
    Diamond3 = 23,
    Diamond2 = 24,
    Diamond1 = 25,
    Ruby5 = 26,
    Ruby4 = 27,
    Ruby3 = 28,
    Ruby2 = 29,
    Ruby1 = 30,
}
