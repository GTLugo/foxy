use std::sync::Arc;

use ash::{
  khr::{self, win32_surface},
  vk::{self, HINSTANCE, HWND},
};
use witer::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use super::{instance::FoxyInstance, RendererError};
use crate::renderer_error;

pub struct FoxySurface {
  surface: vk::SurfaceKHR,
  loader: khr::surface::Instance,
  _instance: Arc<FoxyInstance>,
}

impl Drop for FoxySurface {
  fn drop(&mut self) {
    unsafe { self.loader.destroy_surface(self.surface, None) };
  }
}

impl FoxySurface {
  pub fn new(window: &impl HasRawWindowHandle, instance: Arc<FoxyInstance>) -> Result<Self, RendererError> {
    let RawWindowHandle::Win32(window_handle) = window.raw_window_handle() else {
      return Err(renderer_error!("invalid window handle"));
    };
    let surface = unsafe {
      let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
        .hwnd(window_handle.hwnd as *mut HWND as HWND)
        .hinstance(window_handle.hinstance as *mut HINSTANCE as HINSTANCE);
      let surface_fn = win32_surface::Instance::new(instance.entry(), instance.instance());
      surface_fn.create_win32_surface(&surface_desc, None)
    }?;

    let loader = khr::surface::Instance::new(instance.entry(), instance.instance());

    Ok(Self {
      surface,
      loader,
      _instance: instance,
    })
  }

  pub fn surface(&self) -> &vk::SurfaceKHR {
    &self.surface
  }

  pub fn loader(&self) -> &khr::surface::Instance {
    &self.loader
  }

  pub fn swapchain_support(&self, physical_device: vk::PhysicalDevice) -> Result<SwapchainSupport, RendererError> {
    Ok(SwapchainSupport {
      capabilities: unsafe {
        self
          .loader()
          .get_physical_device_surface_capabilities(physical_device, *self.surface())
      }?,
      formats: unsafe {
        self
          .loader()
          .get_physical_device_surface_formats(physical_device, *self.surface())
      }?,
      present_modes: unsafe {
        self
          .loader()
          .get_physical_device_surface_present_modes(physical_device, *self.surface())
      }?,
    })
  }
}

#[derive(Default)]
pub struct SwapchainSupport {
  pub capabilities: vk::SurfaceCapabilitiesKHR,
  pub formats: Vec<vk::SurfaceFormatKHR>,
  pub present_modes: Vec<vk::PresentModeKHR>,
}
