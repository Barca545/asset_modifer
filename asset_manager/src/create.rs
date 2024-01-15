use std::sync::mpsc::Receiver;

use gl::{DEPTH_TEST, LESS, STENCIL_TEST, NOTEQUAL, KEEP, REPLACE, Gl};
use glfw::{
  fail_on_errors, Context, Glfw, OpenGlProfileHint, Window, WindowEvent,
  WindowHint::{ContextVersionMajor, ContextVersionMinor, OpenGlProfile}
};

use crate::{ecs::{world_resources::ScreenDimensions, World, component_lib::{NormalMesh, Asset, Position}}, filesystem::load_object, polygons::{map_object, Grid}, render::GridMesh};

// #[derive(Debug, Clone, Copy)]
// pub struct ScreenDimensions {
//   pub height:i32,
//   pub width:i32,
//   pub aspect:f32
// }

// impl ScreenDimensions {
//   pub fn new(height:i32, width:i32) -> Self {
//     let aspect = width as f32 / height as f32;
//     ScreenDimensions { height, width, aspect }
//   }
// }

pub fn create_window(world:&World) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
  let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
  
  let mut glfw = glfw::init(fail_on_errors!()).unwrap();
  glfw.window_hint(ContextVersionMajor(3));
  glfw.window_hint(ContextVersionMinor(3));
  glfw.window_hint(OpenGlProfile(OpenGlProfileHint::Core));
  

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
    gl.ClearColor(0.1, 0.1, 0.1, 1.0);
    gl.Enable(DEPTH_TEST);
  }
  gl
}

pub fn create_asset(name:&str, world:&mut World, grid:&mut Grid, grid_mesh:&mut GridMesh){
  let gl = world.immut_get_resource::<Gl>().unwrap();
  //Load the asset's vertices and indices
  let (asset_vertices, asset_indices) = load_object(name).unwrap();

  //Map the object onto the grid
  map_object(&gl,&asset_vertices, &asset_indices, grid, grid_mesh);

  //Create the asset's mesh 
  let asset_mesh = NormalMesh::new(&gl, asset_vertices, asset_indices, "blank_texture");

  //Add the object to the world
  world
    .create_entity()
    .with_component(Asset).unwrap()
    .with_component(asset_mesh).unwrap()
    .with_component(Position::new(0.0,0.0,0.0)).unwrap();
}