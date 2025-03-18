use actix_web::{App, HttpServer, web};
use std::io::Result;

mod extractors;
mod scopes;
use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                option_env!("SECRET")
                    .unwrap_or("VeryGoodSecret")
                    .to_string(),
            ))
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run() 
    .await 
}
