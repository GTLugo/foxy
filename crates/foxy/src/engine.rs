use std::sync::Arc;

use ash::vk;
use winit::{
  application::ApplicationHandler,
  dpi::PhysicalSize,
  event::WindowEvent,
  event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
  window::{Window, WindowAttributes, WindowId},
};

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

#[derive(Debug)]
pub struct Engine {
  pub window: Arc<Window>,
  pub frame_number: u64,
  pub stop_rendering: bool,
  pub window_extent: vk::Extent2D,
}

impl Engine {
  pub fn builder() -> EngineBuilder {
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

struct EngineCompositor {
  attributes: WindowAttributes,
  window: Option<Arc<Window>>,
  engine: Option<Engine>,
}

impl EngineCompositor {
  fn new(attributes: WindowAttributes) -> Self {
    Self {
      attributes,
      window: None,
      engine: None,
    }
  }

  fn run(mut self) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut self).unwrap();
    self.engine.as_mut().unwrap().cleanup();
  }
}

impl ApplicationHandler for EngineCompositor {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    tracing::trace!("Creating window");
    self.window = Some(Arc::new(event_loop.create_window(self.attributes.clone()).unwrap()));
    self.engine = Some(Engine::new(self.window.as_ref().unwrap().clone()));
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        tracing::trace!("Close requested");
        event_loop.exit();
      }
      WindowEvent::KeyboardInput { event, .. } => {
        tracing::debug!("{event:?}");
      }
      WindowEvent::RedrawRequested => {
        self.engine.as_mut().unwrap().draw();
        // self.window.as_ref().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}
