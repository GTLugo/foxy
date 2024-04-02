use foxy_time::TimeSettings;
use witer::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum Polling {
  Poll,
  #[default]
  Wait,
}

#[derive(Debug, Default)]
pub enum DebugInfo {
  Shown,
  #[default]
  Hidden,
}

pub struct FoxySettings {
  pub time: TimeSettings,
  pub title: String,
  pub size: Size,
  pub window: WindowSettings,
  pub debug_info: DebugInfo,
}

impl Default for FoxySettings {
  fn default() -> Self {
    Self {
      time: Default::default(),
      title: "Foxy Window".to_owned(),
      size: LogicalSize::new(800.0, 500.0).into(),
      window: WindowSettings::default(),
      debug_info: DebugInfo::Hidden,
    }
  }
}

impl FoxySettings {
  pub fn with_window(mut self, title: impl Into<String>, size: impl Into<Size>, window: WindowSettings) -> Self {
    self.title = title.into();
    self.size = size.into();
    self.window = window;
    self
  }

  pub fn with_time(mut self, time: TimeSettings) -> Self {
    self.time = time;
    self
  }

  pub fn with_debug_info(mut self, debug_info: DebugInfo) -> Self {
    self.debug_info = debug_info;
    self
  }
}
