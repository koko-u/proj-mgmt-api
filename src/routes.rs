use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_actix_web::GraphQLRequest;
use async_graphql_actix_web::GraphQLResponse;

use crate::handler::ProjectSchema;

pub fn graphql_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::post().to(index))
            .route("", web::get().to(playground)),
    );
}

async fn index(schema: web::Data<ProjectSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

async fn playground() -> impl Responder {
    let source = playground_source(GraphQLPlaygroundConfig::new("/"));

    HttpResponse::Ok()
        .content_type(mime::TEXT_HTML_UTF_8)
        .body(source)
}
