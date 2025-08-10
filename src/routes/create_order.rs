use actix_web::{HttpResponse, Responder, post, web::Json};

use crate::state::{CreateOrder, CreateOrderResponse};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrder>) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    print!(
        "Creating order: price={}, quantity={}, user_id={}, side={:?}\n",
        price, quantity, user_id, side
    );

    // create order logic
    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("order123"),
    });
}
