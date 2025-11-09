use std::sync::Arc;

use winit::{
  application::ApplicationHandler,
  event::WindowEvent,
  event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
  window::{Window, WindowAttributes, WindowId},
};

use crate::Engine;

pub struct EngineCompositor {
  attributes: WindowAttributes,
  window: Option<Arc<Window>>,
  engine: Option<Engine>,
}

impl EngineCompositor {
  pub fn new(attributes: WindowAttributes) -> Self {
    Self {
      attributes,
      window: None,
      engine: None,
    }
  }

  pub fn run(mut self) {
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
      WindowEvent::Resized(_) => {
        let window = self.window.as_ref().unwrap();
        let engine = self.engine.as_mut().unwrap();

        engine.stop_rendering = window.is_minimized().unwrap_or_default();

        tracing::debug!("stop_rendering: {}", engine.stop_rendering);
      }
      WindowEvent::RedrawRequested => {
        let engine = self.engine.as_mut().unwrap();

        // if engine.stop_rendering {
        //   std::thread::sleep(Duration::from_millis(100));
        // }

        engine.draw();

        if !engine.stop_rendering {
          self.window.as_ref().unwrap().request_redraw();
        }
      }
      _ => (),
    }
  }
}
