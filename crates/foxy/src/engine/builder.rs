use winit::{
  dpi::PhysicalSize,
  window::{Window, WindowAttributes},
};

use crate::engine::compositor::EngineCompositor;

pub struct EngineBuilder {
  attributes: WindowAttributes,
}

impl Default for EngineBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl EngineBuilder {
  pub fn new() -> Self {
    Self {
      attributes: Window::default_attributes(),
    }
  }

  pub fn with_window_extent(mut self, width: u32, height: u32) -> Self {
    self.attributes = self.attributes.with_inner_size(PhysicalSize::new(width, height));
    self
  }

  pub fn with_title(mut self, title: impl Into<String>) -> Self {
    self.attributes = self.attributes.with_title(title);
    self
  }

  pub fn run(self) {
    let compositor = EngineCompositor::new(self.attributes);
    compositor.run();
  }
}
