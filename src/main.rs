use actix_web::{App, HttpServer};

mod routes;
mod state;

use routes::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}