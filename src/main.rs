#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{net::TcpStream, collections::HashMap};

use plugin::{server::{Server, ServerEmpty}, tauri::Tauri};
use tauri::{App, Window};
use client_core::make_answer;

mod plugin;
mod event;
mod service;

fn main() {
    make_answer!(Server::new(), Tauri::new(), event::terminal::Terminal::new, service::terminal::Terminal::new);
    // let mut plugin_map: HashMap<String, Box<dyn core::plugin::PluginTrait>> = HashMap::new();
    // plugin_map.insert("server".to_owned(), Box::new(Server::new()));
    // plugin_map.insert("tauri".to_owned(), Box::new(Tauri::new()));
    // let mut metatron_map: HashMap<String, fn() -> Box<dyn core::metatron::MetatronTrait>> = HashMap::new();
    // metatron_map.insert("terminal_event".to_owned(), event::terminal::Terminal::new);
    // metatron_map.insert("terminal_service".to_owned(), service::terminal::Terminal::new);
    // let main_runtime = core::runtime::Runtime::new(HashMap::new(), plugin_map);
    let mut main_window: Option<Window> = None;
    let mut server_stream: Option<TcpStream> = None;
    let mut main_tauri_app: Option<App> = None;

    // let mut mian_tauri = plugin::tauri::Tauri::new(&mut main_tauri_app, &mut main_window);

    // mian_tauri.init_tauri_app();

    // mian_tauri.init_main_window();

    // main_tauri_app
    //     .expect("msg")
    //     .run(|_app_handle, event| match event {
    //         tauri::RunEvent::ExitRequested { api, .. } => {
    //             api.prevent_exit();
    //         }
    //         _ => {}
    //     });
}
