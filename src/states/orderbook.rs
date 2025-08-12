use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::state::{CreateOrder, Side};

#[derive(Deserialize, Serialize)]
pub struct OrderBook {
    pub last_updated_id: u32,
    pub bids: HashMap<String, Vec<OpenOrder>>,
    pub asks: HashMap<String, Vec<OpenOrder>>,
}

#[derive(Deserialize, Serialize)]
pub struct OpenOrder {
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: Side,
    pub user_id: String,
    pub order_id: String,
    pub filled_quantity: f32,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            last_updated_id: 0,
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    pub fn create_order(&mut self, order: CreateOrder) {
        let order_id = self.last_updated_id.to_string();
        self.last_updated_id += 1;

        match order.side {
            Side::Buy => {
                let open_order = OpenOrder {
                    order_id,
                    price: order.price,
                    filled_quantity: 0.0,
                    quantity: order.quantity,
                    side: Side::Buy,
                    user_id: order.user_id,
                };

                self.bids
                    .entry(order.price.to_string())
                    .or_insert(Vec::new())
                    .push(open_order);
            }
            Side::Sell => {
                let openorder = OpenOrder {
                    order_id,
                    price: order.price,
                    filled_quantity: 0.0,
                    quantity: order.quantity,
                    side: Side::Sell,
                    user_id: order.user_id,
                };

                self.asks
                    .entry(order.price.to_string())
                    .or_insert(Vec::new())
                    .push(openorder);
            }
        }
    }
}
