mod create;
mod render;
mod polygons;
mod math;
mod errors;
mod filesystem;
mod ecs;

use std::{thread, time::Duration};
use create::{create_gl, create_asset};
use ecs::{World, component_lib::{NormalMesh, OutlineMesh, Asset, Position}, systems::render, world_resources::{ShouldRender, ShaderPrograms, ScreenDimensions}};
use filesystem::{load_object, load_object_outline};
use glfw::{Key, Action, Context, MouseButton};
use math::{Transforms, Vec3};
use polygons::{Grid, map_object};
use render::GridMesh;
use crate::{create::create_window, ecs::component_lib::MeshPoint};

//this is where I need to handle the result by displaying it to the user
fn main() {
  //Create the world
  let mut world = World::new();

  //Define the viewport and create the transforms
  let screen_dimensions = ScreenDimensions::new(1280, 720);
  let transforms = Transforms::new(&screen_dimensions.aspect);

  //probably needs to be wrapped in a struct
  let mut dragging = false;
  
  world
    .add_resource(screen_dimensions)
    .add_resource(transforms);
  
  //Init glfw
  let (mut glfw, mut window, events) = create_window(&world);
  let gl = create_gl(&mut window);

  world.add_resource(gl.clone());

  let program = ShaderPrograms::new(&world).unwrap();
  
  world  
    .add_resource(gl.clone())
    .add_resource(dragging)
    .add_resource(ShouldRender::default())
    .add_resource(program);

  world
    //type tags
    .register_component::<Asset>()
    //meshes
    .register_component::<MeshPoint>()
    .register_component::<NormalMesh>()
    .register_component::<GridMesh>()
    .register_component::<OutlineMesh>()
    //position
    .register_component::<Position>()
    .register_component::<Grid>();

  //Create the grid 
  let cell_size = 0.1;
  let mut grid = Grid::new(5, 5, cell_size).unwrap();
  let mut grid_mesh = GridMesh::new(&gl, &grid);

  create_asset("ball", &mut world, &mut grid, &mut grid_mesh);
  
  world
    .create_entity()
    .with_component(grid).unwrap()
    .with_component(grid_mesh).unwrap()
    .with_component(Position::new(0.0,0.0,0.0)).unwrap();
  
  while !window.should_close(){
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
          dragging = true
        },
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
          dragging = false
        },
        glfw::WindowEvent::Key(Key::T, _,  Action::Press, _) => {
          let should_render = world.mut_get_resource::<ShouldRender>().unwrap();
          should_render.asset = !should_render.asset;
        },
        // glfw::WindowEvent::Key(Key::T, _,  Action::Release, _) => {
        //   let should_render = world.mut_get_resource::<ShouldRender>().unwrap();
        //   // should_render.asset = false;
        // },
        _ => {}
      }
    }

    //Render
    render(&world).unwrap();
    
    window.swap_buffers();
    thread::sleep(Duration::from_secs(1/30))
  }
}

//might be able to reuse the code for shading the cells to draw a minimap

  // idk if I need to squash the vertex anymore or if the obj can remain 3d
  //make external assets folder for each project to use
  //update shaders to ouput a given color
  //change lines to white
  //--since the vertices are being colored by the grid creation this means the shader for drawing the lines needs to output a constant white color not the vertex color
  
  
  //I might need a seperate shader to render each square filled in with the proper color
  

  //add the server time mod
  //--for rendering each vert on the outline should get a little circle showing where it is
  //--circles should be movable, selectable, and deletable
  //--moving them should update the location of the vert
  //set up overlay capability
  //add GUI
  //add file system access
  //set up hot reloading
  //add input handling
  //convert mouse pictures into worldspace
  //convert mouse input to world input
  //add scripts to calculate the relevant hitboxes
  //add saving
  //ensure interoperability with the game