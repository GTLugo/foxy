use std::{sync::Arc, time::Duration};

use egui::{epaint::Shadow, RawInput, Rounding, Visuals};
use foxy_renderer::renderer::{render_data::RenderData, Renderer};
use foxy_time::{timer::Timer, Time, TimeSettings};
use tracing::error;
use witer::prelude::*;

use super::{builder::DebugInfo, FoxyResult};

pub struct Foxy {
  pub(crate) time: Time,
  pub(crate) window: Arc<Window>,
  pub(crate) egui_context: egui::Context,
  pub(crate) egui_state: witer::compat::egui::State,

  pub(crate) renderer: Renderer,
  pub(crate) render_data: RenderData,

  pub(crate) debug_info: DebugInfo,
  pub(crate) fps_timer: Timer,
  preferred_visibility: Visibility,
  frame_count: u32,
  is_revealed: bool,
}

impl Foxy {
  pub fn new(
    preferred_visibility: Visibility,
    window: Arc<Window>,
    time_settings: TimeSettings,
    debug_info: DebugInfo,
  ) -> FoxyResult<Self> {
    let egui_context = egui::Context::default();
    let id = egui_context.viewport_id();

    const BORDER_RADIUS: f32 = 6.0;

    let visuals = Visuals {
      window_rounding: Rounding::same(BORDER_RADIUS),
      menu_rounding: Rounding::same(BORDER_RADIUS),
      window_shadow: Shadow::NONE,
      ..Default::default()
    };

    egui_context.set_visuals(visuals);
    let egui_state = window.create_egui_state(egui_context.clone(), id, None);

    let time = time_settings.build();
    let renderer = Renderer::new(window.clone())?;

    Ok(Self {
      time,
      window,
      egui_context,
      egui_state,
      renderer,
      render_data: RenderData::default(),
      debug_info,
      fps_timer: Timer::new(),
      preferred_visibility,
      frame_count: 0,
      is_revealed: false,
    })
  }

  pub(crate) fn handle_input(&mut self, message: &Message) -> bool {
    let response = self.egui_state.on_window_event(&self.window, message);

    if response.repaint {
      self.egui_context.request_repaint();
    }

    response.consumed
  }

  pub(crate) fn take_egui_input(&mut self) -> RawInput {
    self.egui_state.take_egui_input(&self.window)
  }

  pub fn delta_time(&self) -> Duration {
    *self.time.delta()
  }

  pub fn average_delta_time(&self) -> Duration {
    *self.time.average_delta()
  }

  pub fn window(&self) -> &Arc<Window> {
    &self.window
  }

  pub fn key(&self, key: Key) -> KeyState {
    self.window.key(key)
  }

  pub fn mouse(&self, mouse: MouseButton) -> ButtonState {
    self.window.mouse(mouse)
  }

  pub fn shift(&self) -> ButtonState {
    self.window.shift()
  }

  pub fn ctrl(&self) -> ButtonState {
    self.window.ctrl()
  }

  pub fn alt(&self) -> ButtonState {
    self.window.alt()
  }

  pub fn win(&self) -> ButtonState {
    self.window.win()
  }

  pub(crate) fn render(&mut self, full_output: egui::FullOutput) -> bool {
    self
      .egui_state
      .handle_platform_output(&self.window, full_output.platform_output);

    let tris = self
      .egui_context
      .tessellate(full_output.shapes, full_output.pixels_per_point);

    self.render_data.egui_tris = Some(tris);

    if let Err(error) = self.renderer.render(&self.time, &self.render_data) {
      error!("`{error}` Aborting...");
      return false;
    }

    match (self.is_revealed, self.frame_count) {
      (false, 3) => {
        self.window.set_visibility(self.preferred_visibility);
        self.is_revealed = true;
      }
      (false, _) => self.frame_count = self.frame_count.wrapping_add(1),
      _ => (),
    };

    if self.fps_timer.has_elapsed(Duration::from_millis(200)) {
      if let DebugInfo::Shown = self.debug_info {
        let ft = self.time.average_delta_secs();
        self
          .window
          .set_subtitle(format!(" | {:^5.4} s | {:>5.0} FPS", ft, 1.0 / ft));
      }
    }

    true
  }
}
