use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware::Logger, web};
use env_logger::Env;
use scalar_doc::scalar_actix::ActixDocumentation;
use std::io::Result;

mod extractors;
mod scopes;
use scopes::user::user_scope;

#[get("/")]
async fn doc() -> impl Responder {
    ActixDocumentation::new("Actix JWT API Documentation", "/openapi")
        .theme(scalar_doc::Theme::Saturn)
        .service()
}

#[get("/openapi")]
async fn openapi() -> impl Responder {
    let open = include_str!("openapi.json");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(open)
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                option_env!("SECRET")
                    .unwrap_or("VeryGoodSecret")
                    .to_string(),
            ))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(doc)
            .service(openapi)
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
