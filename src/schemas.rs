//! GraphQL schemas

pub mod owner;
pub mod project;
pub mod status;

pub use owner::{CreateOwner, FetchOwner, Owner};
pub use project::{CreateProject, FetchProject, Project};
pub use status::Status;
