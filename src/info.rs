pub struct Info {
    pub stream: std::net::TcpStream,
    pub thread_id: usize,
    pub client_address: std::net::SocketAddr,
}