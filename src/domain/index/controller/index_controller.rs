use home_api::route;

// basic handler that responds with a static string
#[route(method = "get", path = "/")]
pub async fn get_status() -> &'static str {
    "Server Status Ok"
}
