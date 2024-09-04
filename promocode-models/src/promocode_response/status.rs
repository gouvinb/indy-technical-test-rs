use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Status {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "denied")]
    Denied,
}
