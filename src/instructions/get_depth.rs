use actix_web::{get, Responder};


#[get("/depth")]
async fn get_depth() -> impl Responder {
    "Depth data retrieved successfully"
}
