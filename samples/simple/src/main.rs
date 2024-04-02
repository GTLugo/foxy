#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use foxy::prelude::*;

pub struct App;

impl Runnable for App {
  fn settings() -> FoxySettings {
    FoxySettings::default()
      .with_window(
        "App",
        LogicalSize::new(800.0, 500.0),
        WindowSettings::default().with_flow(Flow::Poll),
      )
      .with_debug_info(DebugInfo::Shown)
  }

  fn new(_foxy: &mut Foxy) -> Self {
    Self {}
  }

  fn update(&mut self, _foxy: &mut Foxy, event: &Option<Message>) {
    if let Some(Message::Key { .. }) = event {
      tracing::debug!("UPDATE: {:?}", event)
    }
  }
}

fn main() -> FoxyResult<()> {
  init_log();
  App::run()
}

fn init_log() {
  use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_line_number(true)
        .with_file(true),
    )
    .with(
      tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::ERROR)
        .with_targets([
          (env!("CARGO_CRATE_NAME"), tracing::Level::TRACE),
          ("foxy", tracing::Level::TRACE),
          ("foxy_renderer", tracing::Level::TRACE),
          ("foxy_utils", tracing::Level::TRACE),
          ("witer", tracing::Level::TRACE),
        ]),
    )
    .init();
}
