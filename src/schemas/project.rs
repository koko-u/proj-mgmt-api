use async_graphql::InputObject;
use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde::Serialize;

use super::status::Status;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateProject {
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Clone, InputObject)]
pub struct FetchProject {
    pub _id: String,
}
