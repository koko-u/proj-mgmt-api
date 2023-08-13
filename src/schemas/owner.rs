use async_graphql::InputObject;
use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateOwner {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Clone, InputObject)]
pub struct FetchOwner {
    pub _id: String,
}
