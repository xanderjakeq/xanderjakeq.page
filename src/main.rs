use home_server_lib::router::get_router;
use home_server_lib::startup::get_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    //let address = "127.0.0.1:8080";
    let address = "0.0.0.0:8080"; //make this configurable. use 127 for local, 0.0.0.0 for
                                  //prod/docker
    let router = get_router();

    get_server(address).await.serve(router).await;
}
