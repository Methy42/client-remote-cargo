use std::collections::HashMap;

use super::{metatron::MetatronTrait, plugin::PluginTrait};

pub struct Runtime {
  metatron_map: HashMap<String, Box<dyn MetatronTrait>>,
  plugin_map: HashMap<String, Box<dyn PluginTrait>>
}

impl Runtime {
  pub fn new(metatron_map: HashMap<String, Box<dyn MetatronTrait>>, plugin_map: HashMap<String, Box<dyn PluginTrait>>) -> Runtime {
    Runtime {
      metatron_map,
      plugin_map
    }
  }

  pub fn start() {}
}