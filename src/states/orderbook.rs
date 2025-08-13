use colored::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

use super::state::{OpenOrder, Side};

#[derive(Deserialize, Serialize)]
pub struct OrderBook {
    pub last_updated_id: u32,
    pub bids: HashMap<String, Vec<OpenOrder>>,
    pub asks: HashMap<String, Vec<OpenOrder>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            last_updated_id: 0,
            bids: HashMap::new(),
            asks: HashMap::new(),
        }
    }

    pub fn create_order(
        &mut self,
        price: Decimal,
        quantity: Decimal,
        side: Side,
        user_id: String,
    ) -> OpenOrder {
        let order_id: String = self.last_updated_id.to_string();
        self.last_updated_id += 1;

        let open_order = OpenOrder {
            price,
            quantity,
            side: side.clone(),
            user_id,
            order_id,
            filled_quantity: Decimal::ZERO,
        };

        match side {
            Side::Buy => {
                self.bids
                    .entry(price.to_string())
                    .or_insert(Vec::new())
                    .push(open_order.clone());
            }
            Side::Sell => {
                self.asks
                    .entry(price.to_string())
                    .or_insert(Vec::new())
                    .push(open_order.clone());
            }
        }
        open_order
    }

    // what does the match order do?
    // it gets the details of the current incoming order
    // and matches it with the best available orders in the book
    // it will continue to match until the incoming order is fully filled or no more matches are available
    // it will also update the order book accordingly
    pub fn match_orders(&mut self, mut incoming: OpenOrder) {
        // a loop to process the incoming order
        loop {
            // break the loop if the coming order doesn't have any quantity left
            if incoming.quantity <= Decimal::ZERO {
                println!("Incoming order fully filled, stopping match.");
                break;
            }

            // Determine the best price based on the side of the incoming order
            // is_match will be true if the incoming order can match with the best price in the book
            let (best_price_str, is_match) = match incoming.side {
                Side::Buy => {
                    if let Some((price, _)) = self.get_best_ask() {
                        let best_price = Decimal::from_str(price).unwrap();
                        (price.clone(), incoming.price >= best_price)
                    } else {
                        println!("{}", "No asks available to match against".red());
                        return; // Handle the case where there are no best asks
                    }
                }
                Side::Sell => {
                    if let Some((price, _)) = self.get_best_bid() {
                        let best_price = Decimal::from_str(price).unwrap();
                        (price.clone(), incoming.price <= best_price)
                    } else {
                        println!("{}", "No bids available to match against".red());
                        return; // Handle the case where there are no best bids
                    }
                }
            };

            // If the incoming order does not match the best price, exit the loop
            if !is_match {
                println!(
                    "Best price {} not matching incoming order price {}.",
                    best_price_str, incoming.price
                );
                return;
            }

            // get the book (bids or asks) based on the side of the incoming order
            let book = match incoming.side {
                Side::Buy => &mut self.asks,
                Side::Sell => &mut self.bids,
            };

            // Get the orders at the best price level
            if let Some(orders) = book.get_mut(&best_price_str) {
                if let Some(first_order) = orders.first_mut() {
                    let trade_qty = incoming.quantity.min(first_order.quantity);
                    println!(
                        "{}",
                        format!(
                            "[Traded] {:?} order {} matched with {:?} order {} at price {} for quantity {}",
                            incoming.side,
                            incoming.order_id,
                            first_order.side,
                            first_order.order_id,
                            best_price_str,
                            trade_qty
                        )
                        .green()
                    );

                    incoming.quantity -= trade_qty;
                    first_order.quantity -= trade_qty;
                    first_order.filled_quantity += trade_qty;

                    println!(
                        "{}",
                        format!(
                            "Remaining incoming qty: {}, matched order remaining qty: {}",
                            incoming.quantity, first_order.quantity
                        )
                        .blue()
                    );

                    if first_order.quantity <= Decimal::ZERO {
                        println!(
                            "{}",
                            format!(
                                "Matched order {} fully filled, removing from book.",
                                first_order.order_id
                            )
                            .blue()
                        );
                        orders.remove(0);
                    }

                    if orders.is_empty() {
                        println!(
                            "{}",
                            format!("Price level {} empty, removing.", best_price_str).red()
                        );
                        book.remove(&best_price_str);
                    }
                }
            }

            // If the incoming order is fully filled, exit the loop
            if incoming.quantity > Decimal::ZERO {
                let book = match incoming.side {
                    Side::Buy => &mut self.bids,
                    Side::Sell => &mut self.asks,
                };

                // If the incoming order is not fully filled, add it back to the book
                book.entry(incoming.price.to_string())
                    .or_default()
                    .push(incoming);
                // exit the loop since we have added the remaining quantity back to the book
                break;
            }
        }
    }

    pub fn cancel_order(
        &mut self,
        price: Decimal,
        order_id: String,
        side: Side,
    ) -> Result<String, String> {
        let price_str = price.to_string();

        // getting the orders based on the side
        let orders_map = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        // finding the individaul order from the list
        if let Some(order_list) = orders_map.get_mut(&price_str) {
            if let Some(pos) = order_list.iter().position(|o| o.order_id == order_id) {
                order_list.remove(pos);

                if order_list.is_empty() {
                    orders_map.remove(&price_str);
                }
                println!(
                    "{}",
                    format!("Order {} cancelled succesfully", order_id).green()
                );

                return Ok(format!("Order {} canceled successfully", order_id));
            } else {
                println!(
                    "{}",
                    format!("Order {} already canceled or not found", order_id).red()
                );

                return Err(format!("Order {} already canceled or not found", order_id));
            }
        }

        Err(format!(
            "No orders found at price {} for {:?} side",
            price, side
        ))
    }

    pub fn get_best_ask(&mut self) -> Option<(&String, &mut Vec<OpenOrder>)> {
        self.asks
            .iter_mut()
            .min_by_key(|price| Decimal::from_str(price.0).unwrap())
    }

    pub fn get_best_bid(&mut self) -> Option<(&String, &mut Vec<OpenOrder>)> {
        self.bids
            .iter_mut()
            .max_by_key(|price| Decimal::from_str(price.0).unwrap())
    }
}
