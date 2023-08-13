use std::env;
use std::fs;
use std::net;

use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use env_logger::Env;
use proj_mgmt_api::handler::create_project_schema;
use proj_mgmt_api::routes::graphql_routes;
use proj_mgmt_api::AppErr;
use proj_mgmt_api::DBMongo;
use proj_mgmt_api::IntoAppErr;

#[actix_web::main]
async fn main() -> error_stack::Result<(), AppErr> {
    let mut schema_file = env::current_dir().into_apperr()?;
    schema_file.push("schema.graphql");

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("GraphQL Playground on {addr:?}");

    let db = DBMongo::init().await?;
    let schema = create_project_schema(db);
    fs::write(schema_file, schema.sdl()).into_apperr()?;

    let data = web::Data::new(schema);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(data.clone())
            .configure(graphql_routes)
    })
    .bind(addr)
    .into_apperr()?
    .run()
    .await
    .into_apperr()?;

    Ok(())
}
