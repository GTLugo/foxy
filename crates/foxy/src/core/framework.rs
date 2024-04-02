use std::sync::Arc;

use egui::RawInput;
use tracing::*;
use witer::prelude::*;

use super::{builder::FoxySettings, runnable::Runnable, state::Foxy, FoxyResult};
use crate::core::runnable::Flow;

pub struct Framework<App: Runnable> {
  foxy: Foxy,
  app: App,
}

impl<App: Runnable> Framework<App> {
  pub fn new(settings: FoxySettings) -> FoxyResult<Self> {
    let window = Arc::new(
      WindowBuilder::from(
        settings
          .window
          .clone()
          .with_visibility(Visibility::Hidden)
          .with_close_on_x(false),
      )
      .with_title(settings.title.clone())
      .with_size(settings.size)
      .build()?,
    );

    Self::initialize(settings, window)
  }
}

impl<App: Runnable> Framework<App> {
  fn initialize(settings: FoxySettings, window: Arc<Window>) -> FoxyResult<Self> {
    trace!("Firing up Foxy");

    // let (message_sender, message_receiver) = crossbeam::channel::unbounded();

    // let sync_barrier = Arc::new(Barrier::new(2));
    let mut foxy = Foxy::new(settings.window.visibility, window.clone(), settings.time, settings.debug_info)?;
    let app = App::new(&mut foxy);

    // let game_thread = Self::game_loop::<App>(
    //   settings.window.visibility,
    //   window.clone(),
    //   settings.time,
    //   settings.debug_info,
    //   sync_barrier.clone(),
    //   message_receiver,
    // )?;

    Ok(Self { foxy, app })
  }

  pub fn run(mut self) -> FoxyResult<()> {
    info!("KON KON KITSUNE!");

    debug!("Kicking off game loop");

    self.app.start(&mut self.foxy);

    let window = self.foxy.window.clone();
    for message in window.as_ref() {
      // let raw_input: RawInput = self.foxy.take_egui_raw_input();

      match &message {
        Message::Resized(..) => {
          self.foxy.renderer.resize();
        }
        Message::CloseRequested if self.app.stop(&mut self.foxy) == Flow::Exit => {
          self.foxy.window.close();
        }
        Message::Loop(LoopMessage::Exit) => break,
        _ => (),
      }

      let handled = self.foxy.handle_input(&message);

      let message = if handled { None } else { Some(message) };

      self.foxy.time.update();
      while self.foxy.time.should_do_tick_unchecked() {
        self.foxy.time.tick();
        self.app.fixed_update(&mut self.foxy, &message);
      }
      self.app.update(&mut self.foxy, &message);

      let raw_input: RawInput = self.foxy.take_egui_input();
      let full_output = self.foxy.egui_context.run(raw_input, |ui| {
        self.app.egui(&self.foxy, ui);
      });

      if !self.foxy.render(full_output) {
        self.foxy.window.close();
      }
    }

    debug!("Wrapping up game loop");

    self.app.delete();
    self.foxy.renderer.delete();

    // self.game_thread.join().map_err(|e| foxy_error!("{e:?}"))??;
    // self.renderer.delete();

    info!("OTSU KON DESHITA!");

    Ok(())
  }

  // fn game_loop(
  //   &self,
  //   preferred_visibility: Visibility,
  //   window: Arc<Window>,
  //   time_settings: TimeSettings,
  //   debug_info: DebugInfo,
  //   sync_barrier: Arc<Barrier>,
  //   message_receiver: Receiver<Message>,
  // ) -> FoxyResult<JoinHandle<FoxyResult<()>>> {
  //   let handle = std::thread::Builder::new()
  //     .name(Self::GAME_THREAD_ID.into())
  //     .spawn(move || -> FoxyResult<()> {
  //       debug!("Kicking off game loop");

  //       loop {
  //         let message = message_receiver.try_recv().ok();

  //         sync_barrier.wait();
  //       }

  //       trace!("Exiting game!");

  //       debug!("Wrapping up game loop");
  //       Ok(())
  //     })?;

  //   Ok(handle)
  // }
}
