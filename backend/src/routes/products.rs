use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json,
    Router,
};

use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::product::{CreateProduct, Product};

pub type ProductStore = Arc<Mutex<Vec<Product>>>;

pub fn product_routes() -> Router<ProductStore> {
    Router::new()
        .route("/api/products", get(get_products).post(create_product))
        .route("/api/products/{id}", get(get_product))
}

async fn get_products(
    State(store): State<ProductStore>,
) -> Json<Vec<Product>> {
    let products = store.lock().unwrap();
    Json(products.clone())
}

async fn get_product(
    State(store): State<ProductStore>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    let products = store.lock().unwrap();

    let product = products
        .iter()
        .find(|p| p.id == id)
        .cloned();

    match product {
        Some(product) => Ok(Json(product)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_product(
    State(store): State<ProductStore>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    if payload.name.trim().is_empty()
        || payload.sku.trim().is_empty()
        || !payload.price.is_finite()
        || payload.price < 0.0
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let product = Product {
        id: Uuid::new_v4(),
        sku: payload.sku,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        stock_quantity: 0,
    };

    let mut products = store.lock().unwrap();

    if products.iter().any(|p| p.sku == product.sku) {
        return Err(StatusCode::CONFLICT);
    }

    products.push(product.clone());

    Ok((StatusCode::CREATED, Json(product)))
}