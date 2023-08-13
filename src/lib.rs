mod config;
mod error;
pub mod handler;
pub mod schemas;
pub mod routes;

pub use config::mongo::DBMongo;
pub use error::AppErr;
pub use error::IntoAppErr;
pub use error::IntoGraphQLErr;