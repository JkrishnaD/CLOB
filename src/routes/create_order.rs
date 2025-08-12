use std::sync::{Arc, Mutex};

use actix_web::{
    HttpResponse, Responder, post,
    web::{Data, Json},
};

use crate::states::{
    orderbook::OrderBook,
    state::{CreateOrderResponse, OpenOrder},
};

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
        "Parsed order details -> Price: {}, Quantity: {}, User ID: {}, Side: {:?}",
        price, quantity, user_id, side
    );

    // Lock the orderbook
    let mut ob = match orderbook.lock() {
        Ok(guard) => {
            println!("[DEBUG] OrderBook lock acquired");
            guard
        }
        Err(e) => {
            println!("[ERROR] Failed to acquire lock: {}", e);
            return HttpResponse::InternalServerError().body("OrderBook lock error");
        }
    };

    println!("Creating order in orderbook...");
    let incoming_order = ob.create_order(price, quantity, side.clone(), user_id);
    println!("Created order: {:?}", incoming_order);

    println!("Matching orders...");
    ob.match_orders(incoming_order.clone());
    println!("Order matching complete");

    HttpResponse::Ok().json(CreateOrderResponse {
        order_id: incoming_order.order_id,
    })
}
