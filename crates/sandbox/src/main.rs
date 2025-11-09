use tracing::Level;

fn main() {
  tracing_subscriber::fmt()
    .with_max_level(Level::TRACE)
    .with_thread_names(true)
    .pretty()
    .init();

  foxy::Engine::setup()
    .with_title("Vulkan")
    .with_window_extent(800, 500)
    .run();
}
