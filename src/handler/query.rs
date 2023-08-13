use async_graphql::Context;
use async_graphql::Error;
use async_graphql::FieldResult;
use async_graphql::Object;

use crate::schemas::FetchOwner;
use crate::schemas::FetchProject;
use crate::schemas::Owner;
use crate::DBMongo;
use crate::schemas::Project;

#[derive(Debug)]
pub struct Query;

#[Object(extends)]
impl Query {
    /// get the owner
    async fn owner(&self, context: &Context<'_>, input: FetchOwner) -> FieldResult<Owner> {
        let db = context.data_unchecked::<DBMongo>();
        let owner = db.get_single_owner(&input._id).await?;
        match owner {
            Some(owner) => Ok(owner),
            None => Err(Error::new(format!("owner id {} not found", input._id))),
        }
    }

    /// get all owners
    async fn get_owners(&self, context: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let db = context.data_unchecked::<DBMongo>();
        let owners = db.get_owners().await?;

        Ok(owners)
    }

    /// get the project
    async fn project(&self, context: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let db = context.data_unchecked::<DBMongo>();
        let project = db.get_single_project(&input._id).await?;
        match project {
            Some(project) => Ok(project),
            None => Err(Error::new(format!("project id {} not found", input._id))),
        }
    }

    /// get all projects
    async fn get_projects(&self, context: &Context<'_>) -> FieldResult<Vec<Project>> {
        let db = context.data_unchecked::<DBMongo>();
        let projects = db.get_projects().await?;

        Ok(projects)
    }
}
