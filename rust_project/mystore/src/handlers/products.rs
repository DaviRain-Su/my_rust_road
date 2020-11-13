// use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, Responder};

// use crate::models::product::NewProduct;
use crate::models::product::ProductList;

// This is calling  the list method on ProductList and
// Serializing it to a json response
pub async fn index(_req: HttpRequest) -> impl Responder {
    // let name = _req.match_info().get("name").unwrap_or("world");
    // format!("Hello, {}!", &name)
    HttpResponse::Ok().json(ProductList::list())
}

// pub async fn create(new_product: web::Json<NewProduct>) -> Result<HttpResponse, HttpResponse> {
//     // we call the method create from NewProduct and map an ok status response when
//     // everything  works, but map the error from diesel error
//     // to an internal server error when something fail.
//     new_product
//         .create()
//         .map(|product| HttpResponse::Ok().json(product))
//         .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
// }
