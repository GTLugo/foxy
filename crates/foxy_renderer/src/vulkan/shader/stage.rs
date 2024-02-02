// use std::ffi::CString;

// use ash::vk;
// use strum::{Display, EnumIter};

use crate::vulkan::shader::ShaderDiscriminants;

pub mod compute;
pub mod fragment;
pub mod geometry;
pub mod mesh;
pub mod vertex;

pub trait ShaderStage {
  fn kind() -> ShaderDiscriminants;
  fn default_source() -> String;
}
