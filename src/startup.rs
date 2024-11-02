use salvo::conn::tcp::TcpAcceptor;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};

pub async fn get_server(addr: &str) -> Server<TcpAcceptor> {
    let acceptor = TcpListener::new(addr).bind().await;
    return Server::new(acceptor);
}