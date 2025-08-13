use std::sync::{Arc, Mutex};

use actix_web::{
    HttpResponse, Responder, delete,
    web::{Data, Json},
};

use crate::states::{
    orderbook::OrderBook,
    state::{CancelOrder, CancelOrderResponse},
};

#[delete("/order")]
pub async fn cancel_order(
    body: Json<CancelOrder>,
    orderbook: Data<Arc<Mutex<OrderBook>>>,
) -> impl Responder {
    let price = body.0.price;
    let order_id = body.0.order_id.clone();
    let side = body.0.side;

    let mut ob = orderbook.lock().unwrap();

    match ob.cancel_order(price, order_id.clone(), side) {
        Ok(msg) => HttpResponse::Ok().json(CancelOrderResponse {
            order_id,
            message: msg,
        }),
        Err(err) => HttpResponse::BadRequest().json(CancelOrderResponse {
            order_id,
            message: err,
        }),
    }
}
