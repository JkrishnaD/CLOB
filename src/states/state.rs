use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct OpenOrder {
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub user_id: String,
    #[serde(skip_deserializing)]
    pub order_id: String,
    #[serde(default)]
    pub filled_quantity: Decimal,
}

#[derive(Deserialize, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteOrder {
    pub filled_qty: u32,
    pub average_price: Decimal,
}
