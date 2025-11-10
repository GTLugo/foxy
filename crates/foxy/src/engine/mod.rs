mod builder;
mod compositor;
mod instance;
mod swapchain;

use std::sync::Arc;

use ash::vk;
use winit::window::Window;

use crate::engine::{builder::EngineBuilder, instance::Vulkan, swapchain::Swapchain};

pub struct Engine {
  pub window: Arc<Window>,
  vulkan: Vulkan,
  swapchain: Swapchain,
  frame_number: u64,
  stop_rendering: bool,
}

impl Engine {
  pub fn setup() -> EngineBuilder {
    EngineBuilder::default()
  }

  fn new(window: Arc<Window>) -> Self {
    tracing::trace!("Initializing engine");

    let vulkan = Vulkan::new(window.clone());
    tracing::info!("Selected Device: `{} | {}`", vulkan.device_name(), vulkan.api_version().as_string());

    let swapchain = Swapchain::new(vulkan.instance.clone(), vulkan.device.clone(), window.clone());

    Self {
      window,
      vulkan,
      swapchain,
      frame_number: 0,
      stop_rendering: false,
    }
  }

  fn cleanup(&mut self) {
    tracing::trace!("Stopping engine");
    self.swapchain.destroy();
    self.vulkan.destroy();
  }

  fn draw(&mut self) {
    // tracing::trace!("Drawing");
  }
}
