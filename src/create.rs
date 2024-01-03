use std::sync::mpsc::Receiver;

use glfw::{
  fail_on_errors, Context, Glfw, OpenGlProfileHint, Window, WindowEvent,
  WindowHint::{ContextVersionMajor, ContextVersionMinor, OpenGlProfile}
};

#[derive(Debug, Clone, Copy)]
pub struct ScreenDimensions {
  pub height:i32,
  pub width:i32,
  pub aspect:f32
}

impl ScreenDimensions {
  pub fn new(height:i32, width:i32) -> Self {
    let aspect = width as f32 / height as f32;
    ScreenDimensions { height, width, aspect }
  }
}

pub fn create_window() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
  let mut glfw = glfw::init(fail_on_errors!()).unwrap();
  glfw.window_hint(ContextVersionMajor(3));
  glfw.window_hint(ContextVersionMinor(3));
  glfw.window_hint(OpenGlProfile(OpenGlProfileHint::Core));

  let screen_dimensions = ScreenDimensions::new(720, 1280);

  let (mut window, events) = glfw
    .create_window(
      screen_dimensions.width as u32,
      screen_dimensions.height as u32,
      "Asset Manager",
      glfw::WindowMode::Windowed
    )
    .expect("Failed to create GLFW window.");
  window.make_current();
  window.set_all_polling(true);

  (glfw, window, events)
} 