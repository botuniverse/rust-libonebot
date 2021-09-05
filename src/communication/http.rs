use std::net::SocketAddr;

#[derive(Clone)]
pub struct HTTP {
    socket_addr: SocketAddr,
    route: String,
    access_token: String,
}
