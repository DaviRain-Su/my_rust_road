use actix_web::web;
use actix_web::{HttpRequest, HttpResponse};

use crate::models::product::NewProduct;
use crate::models::product::Product;
use crate::models::product::ProductList;
use crate::db_connection::{MysqlPool, MysqlPooledConnection};

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MysqlPooledConnection, HttpResponse> {
    pool.get().map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

// This is calling  the list method on ProductList and
// Serializing it to a json response
pub async fn index(_req: HttpRequest, pool : web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(ProductList::list(&mysql_pool)))
}

// create a product
pub async fn create(new_product: web::Json<NewProduct>, pool: web::Data<MysqlPool>) -> Option<HttpResponse> {
    // we call the method create from NewProduct and map an ok status response when
    // everything  works, but map the error from diesel error
    // to an internal server error when something fail.
    let mysql_pool = mysql_pool_handler(pool).ok()?;

    new_product
        .create(&mysql_pool)
        .map(|product| HttpResponse::Ok().json(product))
    // .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// show product
pub async fn show(id: web::Path<i32>, pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    Product::by_id(&id, &mysql_pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

//delete a product
pub async fn delete(id: web::Path<i32>, pool: web::Data<MysqlPool>) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    Product::delete(&id, &mysql_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub async fn update(
    id: web::Path<i32>,
    new_product: web::Json<NewProduct>,
    pool: web::Data<MysqlPool>
) -> Result<HttpResponse, HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;
    Product::update(&id, &new_product, &mysql_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
