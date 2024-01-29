use foxy_types::{
  behavior::{CloseBehavior, ColorMode, Visibility},
  primitives::Dimensions,
};
use windows::Win32::Foundation::{HINSTANCE, HWND};

use super::input::Input;

#[derive(Debug)]
pub struct WindowState {
  pub hwnd: HWND,
  pub hinstance: HINSTANCE,
  pub size: Dimensions,
  pub inner_size: Dimensions,
  pub title: String,
  pub color_mode: ColorMode,
  pub close_behavior: CloseBehavior,
  pub visibility: Visibility,
  pub input: Input,
}
