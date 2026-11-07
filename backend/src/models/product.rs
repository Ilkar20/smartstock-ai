use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
}