use std::sync::Arc;

use ash::{ext, khr, vk};
use foxy_time::Time;
use gpu_allocator::{
  vulkan::{Allocator, AllocatorCreateDesc},
  AllocationSizes,
  AllocatorDebugSettings,
};
use witer::prelude::*;

use self::{device::FoxyDevice, instance::FoxyInstance, render_data::RenderData, surface::FoxySurface};
use crate::error::RendererError;

mod debug;
mod device;
mod instance;
mod queue;
pub mod render_data;
mod surface;

pub struct Renderer {
  allocator: Allocator,
  device: Arc<FoxyDevice>,
  surface: Arc<FoxySurface>,
  instance: Arc<FoxyInstance>,
  window: Arc<Window>,
}

impl Renderer {
  pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

  pub fn new(window: Arc<Window>) -> Result<Self, RendererError> {
    let instance = Arc::new(FoxyInstance::new()?);
    let surface = Arc::new(FoxySurface::new(&window, instance.clone())?);
    let device = Arc::new(FoxyDevice::new(surface.clone(), instance.clone())?);
    let allocator = Allocator::new(&AllocatorCreateDesc {
      allocation_sizes: AllocationSizes::default(),
      instance: instance.instance().clone(),
      buffer_device_address: true,
      debug_settings: AllocatorDebugSettings::default(),
      device: device.logical().clone(),
      physical_device: *device.physical(),
    })?;

    Ok(Self {
      allocator,
      device,
      surface,
      instance,
      window,
    })
  }

  pub fn delete(&mut self) {}

  pub fn render(&mut self, _render_time: &Time, _render_data: &RenderData) -> Result<(), RendererError> {
    Ok(())
  }

  pub fn resize(&mut self) {}

  pub fn input(&mut self, _message: &Message) -> bool {
    false
  }
}
