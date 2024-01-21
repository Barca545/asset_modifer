use std::{sync::mpsc::Receiver, path::PathBuf};

use gl::{DEPTH_TEST, Gl};
use glfw::{
  fail_on_errors, Context, Glfw, OpenGlProfileHint, Window, WindowEvent,
  WindowHint::{ContextVersionMajor, ContextVersionMinor, OpenGlProfile}
};

use eyre::Result;

use crate::{ecs::{world_resources::ScreenDimensions, World, component_lib::{NormalMesh, Asset, Position}}, filesystem::load_object_with_material, polygons::{Grid, map_object_with_material}, render::GridMesh};

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

pub fn create_asset(pathbuf:PathBuf, world:&mut World) -> Result<()>{
  let gl = world.immut_get_resource::<Gl>().unwrap();

  //really I should I take in the path

  //Load the asset's vertices and indices
  let (asset_vertices, asset_indices, materials) = load_object_with_material(pathbuf).unwrap();

  let mut query = world.query();
  let entities = query.with_component::<Grid>()?.run_entity();

  {
    let entity = &entities[0];

    let mut grid = entity.mut_get_component::<Grid>()?;
    let mut grid_mesh = entity.mut_get_component::<GridMesh>()?;

    //Map the object onto the grid
    map_object_with_material(&gl,&asset_vertices, &asset_indices, &materials, &mut grid, &mut grid_mesh);
  } 

  //Create the asset's mesh 
  let asset_mesh = NormalMesh::new(&gl, asset_vertices, asset_indices, "blank_texture");
  
  //Add the object to the world
  world
    .create_entity()
    .with_component(Asset).unwrap()
    .with_component(asset_mesh).unwrap()
    .with_component(Position::new(0.0,0.0,0.0)).unwrap();

  Ok(())
}