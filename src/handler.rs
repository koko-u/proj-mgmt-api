//! GraphQL logics

mod mutation;
mod query;

use async_graphql::EmptySubscription;
use async_graphql::Schema;

use crate::DBMongo;

pub type ProjectSchema = Schema<query::Query, mutation::Mutation, EmptySubscription>;
pub fn create_project_schema(db: DBMongo) -> ProjectSchema {
    Schema::build(query::Query, mutation::Mutation, EmptySubscription)
        .data(db)
        .finish()
}
