use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
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

impl Default for Store {
    fn default() -> Self {
        Self::new()
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
    // let store = Store::default();
    
    // let store_filter = warp::any().map(move || store.clone());

    // let add_items = warp::post()
    //     .and(warp::path("v1"))
    //     .and(warp::path("groceries"))
    //     .and(warp::path::end())
    //     .and(json_body())
    //     .and(store_filter.clone())
    //     .and_then(add_grocery_list_item);

    // let get_items = warp::get()
    //     .and(warp::path("v1"))
    //     .and(warp::path("groceries"))
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and_then(get_grocery_list);

    // let delete_items = warp::delete()
    //     .and(warp::path("v1"))
    //     .and(warp::path("groceries"))
    //     .and(warp::path::end())
    //     .and(delete_json())
    //     .and(store_filter.clone())
    //     .and_then(delete_grocery_list_item);

    // let update_items = warp::put()
    //     .and(warp::path("v1"))
    //     .and(warp::path("groceries"))
    //     .and(warp::path::end())
    //     .and(post_json())
    //     .and(store_filter.clone())
    //     .and_then(update_grocery_list_item);

    // let routes = add_items.or(get_items).or(delete_items).or(update_items);

    // warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    pretty_env_logger::init();

    // We'll start simple, and gradually show how you combine these powers
    // into super powers!

    // GET /
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    // GET /hi
    let hi = warp::path("hi").map(|| "Hello, World!");

    // How about multiple segments? First, we could use the `path!` macro:
    //
    // GET /hello/from/warp
    let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| "Hello from warp!");

    // Fine, but how do I handle parameters in paths?
    //
    // GET /sum/:u32/:u32
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // Any type that implements FromStr can be used, and in any order:
    //
    // GET /:u16/times/:u16
    let times =
        warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b));

    // Oh shoot, those math routes should be mounted at a different path,
    // is that possible? Yep.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math");
    let _sum = math.and(sum);
    let _times = math.and(times);

    // What! And? What's that do?
    //
    // It combines the filters in a sort of "this and then that" order. In
    // fact, it's exactly what the `path!` macro has been doing internally.
    //
    // GET /bye/:string
    let bye = warp::path("bye")
        .and(warp::path::param())
        .map(|name: String| format!("Good bye, {}!", name));

    // Ah, can filters do things besides `and`?
    //
    // Why, yes they can! They can also `or`! As you might expect, `or` creates
    // a "this or else that" chain of filters. If the first doesn't succeed,
    // then it tries the other.
    //
    // So, those `math` routes could have been mounted all as one, with `or`.
    //
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16
    let math = warp::path("math").and(sum.or(times));

    // We can use the end() filter to match a shorter path
    let help = warp::path("math")
        // Careful! Omitting the following line would make this filter match
        // requests to /math/sum/:u32/:u32 and /math/:u16/times/:u16
        .and(warp::path::end())
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16");
    let math = help.or(math);

    // Let's let people know that the `sum` and `times` routes are under `math`.
    let sum = sum.map(|output| format!("(This route has moved to /math/sum/:u16/:u16) {}", output));
    let times =
        times.map(|output| format!("(This route has moved to /math/:u16/times/:u16) {}", output));

    // It turns out, using `or` is how you combine everything together into
    // a single API. (We also actually haven't been enforcing that the
    // method is GET, so we'll do that too!)
    //
    // GET /
    // GET /hi
    // GET /hello/from/warp
    // GET /bye/:string
    // GET /math/sum/:u32/:u32
    // GET /math/:u16/times/:u16

    let routes = warp::get().and(
        hello_world
            .or(hi)
            .or(hello_from_warp)
            .or(bye)
            .or(math)
            .or(sum)
            .or(times),
    );

    // Note that composing filters for many routes may increase compile times (because it uses a lot of generics).
    // If you wish to use dynamic dispatch instead and speed up compile times while
    // making it slightly slower at runtime, you can use Filter::boxed().

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
