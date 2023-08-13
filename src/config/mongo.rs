use mongodb::Client;
use mongodb::Collection;
use mongodb::Database;

use crate::error::AppErr;
use crate::error::IntoAppErr;

#[derive(Debug)]
pub struct DBMongo(Database);

impl DBMongo {
    pub async fn init() -> error_stack::Result<Self, AppErr> {
        dotenv::dotenv().into_apperr()?;
        let mongo_uri = dotenv_codegen::dotenv!("MONGO_URI");
        let client = Client::with_uri_str(mongo_uri).await.into_apperr()?;
        let db = client.database("projMgmt");

        Ok(Self(db))
    }

    fn col_helper<T>(&self, collection_name: &str) -> Collection<T> {
        self.0.collection(collection_name)
    }
}

mod impl_owner;
mod impl_project;
