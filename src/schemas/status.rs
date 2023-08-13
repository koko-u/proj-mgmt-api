use async_graphql::Enum;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}
