use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock_quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64
}