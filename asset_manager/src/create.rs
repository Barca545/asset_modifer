use std::sync::mpsc::Receiver;

use gl::{DEPTH_TEST, LESS, STENCIL_TEST, NOTEQUAL, KEEP, REPLACE, Gl};
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

pub fn create_gl(window:&mut Window) -> Gl {
  let _gl_context = window.get_context_version();
  let gl = Gl::load_with(&mut |s| window.get_proc_address(s) as *const std::os::raw::c_void);
  unsafe {
    gl.Enable(DEPTH_TEST);
    gl.DepthFunc(LESS);
    gl.Enable(STENCIL_TEST);
    gl.StencilFunc(NOTEQUAL, 1, 0xFF);
    gl.StencilOp(KEEP, KEEP, REPLACE);
  }
  gl
}