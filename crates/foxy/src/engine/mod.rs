mod builder;
mod compositor;

use std::sync::Arc;

use ash::vk;
use winit::window::Window;

use crate::engine::builder::EngineBuilder;

#[derive(Debug)]
pub struct Engine {
  pub window: Arc<Window>,
  pub frame_number: u64,
  pub stop_rendering: bool,
  pub window_extent: vk::Extent2D,
}

impl Engine {
  pub fn setup() -> EngineBuilder {
    EngineBuilder::default()
  }

  fn new(window: Arc<Window>) -> Self {
    tracing::trace!("Initializing engine");
    let size = window.inner_size();
    Self {
      frame_number: 0,
      stop_rendering: false,
      window_extent: vk::Extent2D {
        width: size.width,
        height: size.height,
      },
      window,
    }
  }

  fn cleanup(&mut self) {
    tracing::trace!("Stopping engine");
  }

  fn draw(&mut self) {
    tracing::trace!("Drawing");
  }
}
