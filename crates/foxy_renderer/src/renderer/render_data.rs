use std::fmt::Debug;

use egui::ClippedPrimitive;

#[derive(Default)]
pub struct RenderData {
  pub egui_tris: Option<Vec<ClippedPrimitive>>,
}

impl Debug for RenderData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "RenderData {{ .. }}")
  }
}
