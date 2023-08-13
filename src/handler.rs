//! GraphQL logics

mod mutation;
mod query;

use async_graphql::EmptySubscription;
use async_graphql::Schema;

pub type ProjectSchema = Schema<query::Query, mutation::Mutation, EmptySubscription>;
