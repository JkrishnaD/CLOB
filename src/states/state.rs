use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateOrder {
    pub price: Decimal,
    pub quantity: Decimal,
    pub user_id: String,
    pub side: Side,
}

#[derive(Deserialize, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteOrder {
    pub filled_qty: u32,
    pub average_price: Decimal,
}
