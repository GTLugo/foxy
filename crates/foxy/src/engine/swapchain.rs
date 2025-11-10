use std::sync::Arc;

use ash::{khr, vk};
use ash_bootstrap as vkb;
use winit::window::Window;

pub struct Swapchain {
  window: Arc<Window>,
  swapchain: vkb::Swapchain,
}

impl Swapchain {
  pub fn new(instance: Arc<vkb::Instance>, device: Arc<vkb::Device>, window: Arc<Window>) -> Self {
    let size = window.inner_size();
    let extent = vk::Extent2D {
      width: size.width,
      height: size.height,
    };

    let builder = vkb::SwapchainBuilder::new(instance, device);

    let swapchain = builder
      .desired_format(vk::SurfaceFormat2KHR::default().surface_format(vk::SurfaceFormatKHR {
        format: vk::Format::B8G8R8A8_UNORM,
        color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
      }))
      .desired_present_mode(vk::PresentModeKHR::IMMEDIATE)
      .desired_size(extent)
      .add_image_usage_flags(vk::ImageUsageFlags::TRANSFER_DST)
      .build()
      .unwrap();

    Self { window, swapchain }
  }

  pub fn destroy(&self) {
    self.swapchain.destroy();
    self.swapchain.destroy_image_views().unwrap();
  }

  fn window_extent(&self) -> vk::Extent2D {
    let size = self.window.inner_size();
    vk::Extent2D {
      width: size.width,
      height: size.height,
    }
  }
}
