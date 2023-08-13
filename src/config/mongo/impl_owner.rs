use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;

use crate::error::AppErr;
use crate::error::IntoAppErr;
use crate::schemas::Owner;

impl super::DBMongo {
    pub async fn create_owner(&self, new_owner: Owner) -> error_stack::Result<Owner, AppErr> {
        let new_doc = Owner {
            _id: None,
            ..new_owner.clone()
        };
        let col = self.col_helper::<Owner>("owner");

        let data = col.insert_one(new_doc, None).await.into_apperr()?;
        let new_owner = Owner {
            _id: data.inserted_id.as_object_id(),
            ..new_owner.clone()
        };

        Ok(new_owner)
    }

    pub async fn get_owners(&self) -> error_stack::Result<Vec<Owner>, AppErr> {
        let col = self.col_helper::<Owner>("owner");
        let mut cursor = col.find(None, None).await.into_apperr()?;
        let mut owners = Vec::new();
        while let Some(owner) = cursor.try_next().await.into_apperr()? {
            owners.push(owner);
        }

        Ok(owners)
    }

    pub async fn get_single_owner(&self, id: &str) -> error_stack::Result<Option<Owner>, AppErr> {
        let id = id.parse::<ObjectId>().into_apperr()?;
        let filter = bson::doc! { "_id": id };
        let col = self.col_helper::<Owner>("owner");

        let owner = col.find_one(filter, None).await.into_apperr()?;

        Ok(owner)
    }
}
