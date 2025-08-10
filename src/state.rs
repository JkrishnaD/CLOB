use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateOrder {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side,
}

#[derive(Deserialize, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Side {
    Buy,
    Sell,
}
