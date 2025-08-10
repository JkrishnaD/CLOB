use actix_web::{delete, Responder};

#[delete("/order")]
pub async fn delete_order() -> impl Responder {
    "Order deleted successfully"
}