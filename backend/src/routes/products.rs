use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json,
    Router,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::models::product::{
    CreateProduct,
    Product,
    UpdateProduct,
};

pub fn product_routes() -> Router<PgPool> {
    Router::new()
        .route(
            "/api/products",
            get(get_products).post(create_product),
        )
        .route(
            "/api/products/{id}",
            get(get_product)
                .put(update_product)
                .delete(delete_product),
        )
}

fn valid_product(sku: &str, name: &str, price: f64) -> bool {
    !sku.trim().is_empty()
        && !name.trim().is_empty()
        && price.is_finite()
        && price >= 0.0
}

fn database_error(error: sqlx::Error) -> StatusCode {
    if let sqlx::Error::Database(db_error) = &error {
        if db_error.code().as_deref() == Some("23505") {
            return StatusCode::CONFLICT;
        }

        if db_error.code().as_deref() == Some("23514") {
            return StatusCode::BAD_REQUEST;
        }
    }

    tracing::error!("Database error: {error}");

    StatusCode::INTERNAL_SERVER_ERROR
}

// GET /api/products
async fn get_products(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let products = sqlx::query_as::<_, Product>(
        "SELECT id, sku, name, description, price
         FROM products
         ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .map_err(database_error)?;

    Ok(Json(products))
}

// GET /api/products/{id}
async fn get_product(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    let product = sqlx::query_as::<_, Product>(
        "SELECT id, sku, name, description, price
         FROM products
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(database_error)?;

    match product {
        Some(product) => Ok(Json(product)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// POST /api/products
async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    if !valid_product(
        &payload.sku,
        &payload.name,
        payload.price,
    ) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = Uuid::new_v4();

    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products
         (id, sku, name, description, price)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, sku, name, description, price"
    )
    .bind(id)
    .bind(payload.sku)
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.price)
    .fetch_one(&pool)
    .await
    .map_err(database_error)?;

    Ok((StatusCode::CREATED, Json(product)))
}

// PUT /api/products/{id}
async fn update_product(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    if !valid_product(
        &payload.sku,
        &payload.name,
        payload.price,
    ) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let product = sqlx::query_as::<_, Product>(
        "UPDATE products
         SET sku = $1,
             name = $2,
             description = $3,
             price = $4
         WHERE id = $5
         RETURNING id, sku, name, description, price"
    )
    .bind(payload.sku)
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.price)
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(database_error)?;

    match product {
        Some(product) => Ok(Json(product)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// DELETE /api/products/{id}
async fn delete_product(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "DELETE FROM products WHERE id = $1"
    )
    .bind(id)
    .execute(&pool)
    .await
    .map_err(database_error)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}