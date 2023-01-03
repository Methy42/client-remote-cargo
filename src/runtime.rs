use std::net::TcpStream;

use tauri::Window;

pub struct Runtime<'a> {
    pub main_window: &'a mut Option<Window>,
    pub server_stream: &'a mut Option<TcpStream>,
}