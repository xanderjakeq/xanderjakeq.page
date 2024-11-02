use salvo::conn::tcp::TcpAcceptor;
use salvo::conn::TcpListener;
use salvo::{Listener, Server};

pub async fn server_at(addr: &str) -> Server<TcpAcceptor> {
    let acceptor = TcpListener::new(addr).bind().await;
    return Server::new(acceptor);
}
