use async_graphql::Context;
use async_graphql::FieldResult;
use async_graphql::Object;

use crate::schemas::CreateOwner;
use crate::schemas::CreateProject;
use crate::schemas::Owner;
use crate::schemas::Project;
use crate::DBMongo;

#[derive(Debug)]
pub struct Mutation;

#[Object]
impl Mutation {
    /// create owner
    async fn create_owner(&self, context: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
        let db = context.data_unchecked::<DBMongo>();
        let new_owner = Owner {
            _id: None,
            name: input.name,
            email: input.email,
            phone: input.phone,
        };
        let owner = db.create_owner(new_owner).await?;

        Ok(owner)
    }

    /// create project
    async fn create_project(
        &self,
        context: &Context<'_>,
        input: CreateProject,
    ) -> FieldResult<Project> {
        let db = context.data_unchecked::<DBMongo>();
        let new_project = Project {
            _id: None,
            owner_id: input.owner_id,
            name: input.name,
            description: input.description,
            status: input.status,
        };
        let project = db.create_project(new_project).await?;

        Ok(project)
    }
}
