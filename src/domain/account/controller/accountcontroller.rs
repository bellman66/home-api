use home_api::route;
use crate::HASHMAP;

// basic handler that responds with a static string
#[route(method = "get", path = "/")]
pub async fn get_status() -> &'static str {
    // "Server Status OK"
    HASHMAP.get(&0).unwrap()
}
