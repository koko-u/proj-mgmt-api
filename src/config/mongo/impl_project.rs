use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;

use crate::error::AppErr;
use crate::error::IntoAppErr;
use crate::schemas::Project;

impl super::DBMongo {
    pub async fn create_project(
        &self,
        new_project: Project,
    ) -> error_stack::Result<Project, AppErr> {
        let new_doc = Project {
            _id: None,
            ..new_project.clone()
        };
        let col = self.col_helper::<Project>("project");

        let data = col.insert_one(new_doc, None).await.into_apperr()?;
        let new_project = Project {
            _id: data.inserted_id.as_object_id(),
            ..new_project.clone()
        };

        Ok(new_project)
    }

    pub async fn get_projects(&self) -> error_stack::Result<Vec<Project>, AppErr> {
        let col = self.col_helper::<Project>("project");
        let mut cursor = col.find(None, None).await.into_apperr()?;
        let mut projects = Vec::new();
        while let Some(project) = cursor.try_next().await.into_apperr()? {
            projects.push(project);
        }

        Ok(projects)
    }

    pub async fn get_single_project(&self, id: &str) -> error_stack::Result<Option<Project>, AppErr> {
        let id = id.parse::<ObjectId>().into_apperr()?;
        let filter = bson::doc! { "_id": id };
        let col = self.col_helper::<Project>("project");

        let project = col.find_one(filter, None).await.into_apperr()?;

        Ok(project)
    }

}
