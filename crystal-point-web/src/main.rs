use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::{clone, hash::Hash, sync::Arc};
use std::{collections::HashMap, fmt::write};
use warp::{http, Filter};

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
pub struct Store {
    grocery_list: Arc<RwLock<Items>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // when accepting a body , we want a json body
    // and to reject huge payloads
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // when accepting a body, we want a JSON body
    // and to reject huge payloads
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // when accepting a body , we want a json body
    // and to reject huge payloads
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// add our first router
async fn add_grocery_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Add items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

async fn get_grocery_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();

    let r = store.grocery_list.read();

    for (key, value) in r.iter() {
        result.insert(key, value);
    }

    Ok(warp::reply::json(&result))
}

// add our first router
async fn delete_grocery_list_item(
    id: Id,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().remove(&id.name);

    Ok(warp::reply::with_status(
        "Removed items to the grocery list",
        http::StatusCode::OK,
    ))
}

// add our first router
async fn update_grocery_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Update items to the grocery list",
        http::StatusCode::CREATED,
    ))
}
#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_grocery_list_item);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let delete_items = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_grocery_list_item);

    let update_items = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_grocery_list_item);

    let routes = add_items.or(get_items).or(delete_items).or(update_items);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
