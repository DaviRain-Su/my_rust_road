use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::models::product::ProductList;

// This is calling  the list method on ProductList and
// Serializing it to a json response
pub async fn index(_req: HttpRequest) -> impl Responder {
    // let name = _req.match_info().get("name").unwrap_or("world");
    // format!("Hello, {}!", &name)
    HttpResponse::Ok().json(ProductList::list())

}

