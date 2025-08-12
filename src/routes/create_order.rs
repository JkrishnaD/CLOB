use std::sync::{Arc, Mutex};

use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};

use crate::states::{
    orderbook::OrderBook,
    state::{CreateOrder, CreateOrderResponse},
};

#[post("/order")]
pub async fn create_order(
    body: Json<CreateOrder>,
    orderbook: Data<Arc<Mutex<OrderBook>>>,
) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    // here we need to lock the oerder book so that it comes to the scope
    let mut ob = orderbook.lock().unwrap();
    ob.create_order(CreateOrder {
        price,
        quantity,
        user_id,
        side,
    });

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("order123"),
    });
}
