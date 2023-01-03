use std::net::TcpStream;

pub struct Server {
    server_ip: Option<String>,
    server_socket: Option<TcpStream>,
}

pub trait ServerEmpty {
    fn new() -> Server;
}

pub trait ServerWithParams {
    fn new(server_ip: String) -> Server;
}

impl Server {
    pub fn connect_socket(&mut self) {
        match &self.server_ip {
            Some(something) => {
                self.server_socket = Some(TcpStream::connect(something).expect("Couldn't connect to the server..."));
            },
            None => {},
        }
        
    }
}

impl ServerEmpty for Server {
    fn new() -> Server {
        Server { server_ip: None, server_socket: None }
    }
}

impl ServerWithParams for Server {
    fn new(server_ip: String) -> Server {
        Server { server_ip: Some(server_ip), server_socket: None }
    }
}
