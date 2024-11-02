use home_server_lib::router::get_router;
use home_server_lib::startup::get_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let address = "127.0.0.1:3000";
    let router = get_router();

    get_server(address).await.serve(router).await;
}
