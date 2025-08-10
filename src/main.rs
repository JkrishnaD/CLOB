use actix_web::{App, HttpServer, Responder, delete, get, post};

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

#[post("/order")]
async fn create_order() -> impl Responder {
    
    "Order created successfully"
}

#[delete("/order")]
async fn delete_order() -> impl Responder {
    "Order deleted successfully"
}

#[get("/depth")]
async fn get_depth() -> impl Responder {
    "Depth data retrieved successfully"
}
