use thiserror::Error;

#[derive(Error, Debug)]
pub enum RendererError {
  #[error("{0}")]
  Error(String),
  #[error("{0}")]
  LoadingError(#[from] ash::LoadingError),
  #[error("{0}")]
  VkResult(#[from] ash::vk::Result),
  #[error("{0}")]
  AllocError(#[from] gpu_allocator::AllocationError),
  #[error("{0}")]
  IO(#[from] std::io::Error),
}

#[macro_export]
macro_rules! renderer_error {
  () => {
    $crate::error::RendererError::Error("renderer error".to_string())
  };
  ($($arg:tt)*) => {{
    $crate::error::RendererError::Error(format!($($arg)*))
  }}
}
