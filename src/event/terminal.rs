use crate::{plugin::{tauri::EventTrait}};
use client_core::Metatron;

#[derive(Metatron)]
pub struct Terminal {}

impl Terminal {
    pub fn new(terminal_service: crate::service::terminal::Terminal) -> Terminal {
      Terminal {}
    }
}

impl EventTrait for Terminal {}