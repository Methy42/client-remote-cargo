use super::runtime::Runtime;

pub trait PluginTrait {
  fn init(&mut self, runtime: &Runtime);

  fn start(&mut self, runtime: &Runtime);

  fn stop(&mut self, runtime: &Runtime);
}