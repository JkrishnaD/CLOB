use std::sync::{Arc, Mutex};

use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};

use crate::states::{
    orderbook::OrderBook,
    state::{CreateOrderResponse, OpenOrder},
};
use colored::*;

#[post("/order")]
pub async fn create_order(
    body: Json<OpenOrder>,
    orderbook: Data<Arc<Mutex<OrderBook>>>,
) -> impl Responder {
    println!("Received order request: {:?}", body);

    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    println!(
        "{}",
        format!(
            "Parsed order details -> Price: {}, Quantity: {}, User ID: {}, Side: {:?}",
            price, quantity, user_id, side
        )
        .blue()
    );

    // Lock the orderbook
    let mut ob = match orderbook.lock() {
        Ok(guard) => {
            println!("{}", "OrderBook lock acquired".red());
            guard
        }
        Err(e) => {
            println!("Failed to acquire lock: {}", e);
            return HttpResponse::InternalServerError().body("OrderBook lock error");
        }
    };

    let incoming_order = ob.create_order(price, quantity, side.clone(), user_id);
    println!("Created order: {:?}", incoming_order);

    println!("Matching orders...");
    ob.match_orders(incoming_order.clone());
    println!("Order matching complete");

    HttpResponse::Ok().json(CreateOrderResponse {
        order_id: incoming_order.order_id,
    })
}
