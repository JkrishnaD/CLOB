use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, web::Data};
use colored::Colorize;

use crate::states::orderbook::OrderBook;

#[get("/depth")]
async fn get_depth(orderbook: Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    // Lock the orderbook
    let ob = match orderbook.lock() {
        Ok(guard) => {
            println!("{}", "OrderBook lock acquired".red());
            guard
        }
        Err(e) => {
            println!("Failed to acquire lock: {}", e);
            return HttpResponse::InternalServerError().body("OrderBook lock error");
        }
    };

    let depth = ob.get_depth();
    println!("{}", depth.green());
    HttpResponse::Ok().json(depth.to_string())
}
