use home_server_lib::router::get_router;
use home_server_lib::startup::server_at;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let address = "127.0.0.1:3000";
    let router = get_router();

    server_at(address).await.serve(router).await;
}
