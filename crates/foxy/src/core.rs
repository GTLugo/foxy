use self::{
  builder::{FoxyBuilder, FoxyCreateInfo, HasSize, HasTitle, MissingSize, MissingTitle},
  lifecycle::Lifecycle,
};
use foxy_renderer::renderer::{render_data::RenderData, Renderer};
use foxy_util::time::{EngineTime, Time};
use foxy_window::prelude::*;
use message::{GameLoopMessage, RenderLoopMessage};
use messaging::Mailbox;
use std::{mem, thread::JoinHandle};
use tracing::*;

pub mod builder;
pub mod lifecycle;
mod message;

pub struct Foxy {
  time: EngineTime,
  current_state: Lifecycle,
  window: Window,
  render_thread: Option<JoinHandle<anyhow::Result<()>>>,
  game_mailbox: Mailbox<GameLoopMessage, RenderLoopMessage>,
}

impl Foxy {
  pub const RENDER_THREAD_ID: &'static str = "render";

  pub fn builder() -> FoxyBuilder<MissingTitle, MissingSize> {
    Default::default()
  }

  pub fn new(foxy_create_info: FoxyCreateInfo<HasTitle, HasSize>) -> anyhow::Result<Self> {
    let time = EngineTime::new(128.0, 1024);

    let mut window = Window::builder()
      .with_title(foxy_create_info.title.0)
      .with_size(foxy_create_info.size.width, foxy_create_info.size.height)
      .with_color_mode(foxy_create_info.color_mode)
      .with_close_behavior(foxy_create_info.close_behavior)
      .with_visibility(Visibility::Hidden)
      .build()?;

    let renderer = Renderer::new(&window)?;
    window.set_visibility(Visibility::Shown);

    let (renderer_mailbox, game_mailbox) = Mailbox::new_entangled_pair();
    let render_thread = Self::render_loop(renderer, renderer_mailbox)
      .inspect_err(|e| error!("{e}"))
      .ok();

    let current_state = Lifecycle::Entering;

    Ok(Self {
      time,
      current_state,
      window,
      render_thread,
      game_mailbox,
    })
  }

  pub fn time(&self) -> &Time {
    self.time.time()
  }

  pub fn window(&self) -> &Window {
    &self.window
  }

  pub fn poll(&mut self) -> Option<&Lifecycle> {
    self.next_state(false)
  }

  pub fn wait(&mut self) -> Option<&Lifecycle> {
    self.next_state(true)
  }

  fn next_state(&mut self, should_wait: bool) -> Option<&Lifecycle> {
    let new_state = match &mut self.current_state {
      Lifecycle::Entering => {
        let message = if should_wait {
          self.window.wait()
        } else {
          self.window.next()
        };
        if let Some(message) = message {
          Lifecycle::BeginFrame { message }
        } else {
          Lifecycle::Exiting
        }
      }
      Lifecycle::BeginFrame { message } => {
        let message = mem::take(message);
        if let Err(error) = self.game_mailbox.send_and_wait(GameLoopMessage::SyncWithRenderer) {
          error!("{error}");
          Lifecycle::Exiting
        } else {
          Lifecycle::EarlyUpdate { message }
        }
      }
      Lifecycle::EarlyUpdate { message } => {
        let message = mem::take(message);
        self.time.update();
        if self.time.should_do_tick() {
          self.time.tick();
          Lifecycle::FixedUpdate { message }
        } else {
          Lifecycle::Update { message }
        }
      }
      Lifecycle::FixedUpdate { message } => {
        let message = mem::take(message);
        if self.time.should_do_tick() {
          self.time.tick();
          Lifecycle::FixedUpdate { message }
        } else {
          Lifecycle::Update { message }
        }
      }
      Lifecycle::Update { message } => {
        let message = mem::take(message);
        Lifecycle::EndFrame { message }
      }
      Lifecycle::EndFrame { message } => {
        let message = mem::take(message);
        match self.game_sync_or_exit(&message) {
          Ok(value) => {
            if value {
              Lifecycle::Exiting
            } else {
              let message = if should_wait {
                self.window.wait()
              } else {
                self.window.next()
              };
              if let Some(message) = message {
                Lifecycle::BeginFrame { message }
              } else {
                Lifecycle::Exiting
              }
            }
          }
          Err(error) => {
            error!("{error}");
            Lifecycle::Exiting
          }
        }
      }
      Lifecycle::Exiting => Lifecycle::ExitLoop,
      Lifecycle::ExitLoop => return None,
    };

    self.current_state = new_state;

    // debug!("{:?}", self.current_state);
    Some(&self.current_state)
  }

  fn game_sync_or_exit(&mut self, received_message: &WindowMessage) -> anyhow::Result<bool> {
    match received_message {
      WindowMessage::Closed => {
        self.game_mailbox.send_and_wait(GameLoopMessage::Exit)?;
        if let Some(thread_handle) = self.render_thread.take() {
          if let Err(error) = thread_handle.join() {
            error!("{error:?}");
          }
        }
        Ok(true)
      }
      _ => {
        self
          .game_mailbox
          .send_and_wait(GameLoopMessage::RenderData(RenderData {}))?;
        Ok(false)
      }
    }
  }

  fn render_loop(
    mut renderer: Renderer,
    mut messenger: Mailbox<RenderLoopMessage, GameLoopMessage>,
  ) -> anyhow::Result<JoinHandle<anyhow::Result<()>>> {
    std::thread::Builder::new()
      .name(Self::RENDER_THREAD_ID.into())
      .spawn(move || -> anyhow::Result<()> {
        trace!("Beginning render");

        loop {
          if Self::renderer_sync_or_exit(&mut renderer, &mut messenger)? {
            break;
          }

          renderer.render()?;

          if Self::renderer_sync_or_exit(&mut renderer, &mut messenger)? {
            break;
          }
        }

        trace!("Ending render");

        Ok(())
      })
      .map_err(anyhow::Error::from)
  }

  fn renderer_sync_or_exit(
    renderer: &mut Renderer,
    messenger: &mut Mailbox<RenderLoopMessage, GameLoopMessage>,
  ) -> anyhow::Result<bool> {
    match messenger.send_and_wait(RenderLoopMessage::SyncWithGame)? {
      GameLoopMessage::SyncWithRenderer => Ok(false),
      GameLoopMessage::RenderData(data) => {
        renderer.update_render_data(data)?;
        Ok(false)
      }
      GameLoopMessage::Exit => Ok(true),
    }
  }
}
