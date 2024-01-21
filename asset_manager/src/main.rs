mod create;
mod render;
mod polygons;
mod math;
mod errors;
mod filesystem;
mod ecs;
mod input_manager;

use std::{thread, time::Duration};
use create::{create_gl, create_asset};
use ecs::{World, component_lib::{NormalMesh, OutlineMesh, Asset, Position}, systems::{render, input}, world_resources::{ShouldRender, ShaderPrograms, ScreenDimensions}};
use filesystem::open_file_dialog;
use glfw::{Key, Action, Context, MouseButton};
use input_manager::{InputManager, MouseRay, Mode, DrawMode};
use math::Transforms;
use polygons::{Grid, Terrain};
use render::GridMesh;
use crate::{create::create_window, ecs::component_lib::MeshPoint};

//this is where I need to handle the result by displaying it to the user
fn main() {
  //Create the world
  let mut world = World::new();

  //Define the viewport and create the transforms
  let screen_dimensions = ScreenDimensions::new(1280, 720);
  let transforms = Transforms::new(&screen_dimensions.aspect);
  
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
    .add_resource(ShouldRender::default())
    .add_resource(program)
    .add_resource(InputManager::new());

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
  //this can go into one function
  let cell_size = 0.2;
  let grid = Grid::new(20, 20, cell_size).unwrap();
  let grid_mesh = GridMesh::new(&gl, &grid);

  world
    .create_entity()
    .with_component(grid).unwrap()
    .with_component(grid_mesh).unwrap()
    .with_component(Position::new(0.0,0.0,0.0)).unwrap();

  //I think I either need to flatten the object when I load it in *or* use a different projection 
  //the current projection causes problematic distortion
  // create_asset("map_test", &mut world).unwrap();
  
  while !window.should_close(){
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::Key(Key::T, _,  Action::Press, _) => {
          let should_render = world.mut_get_resource::<ShouldRender>().unwrap();
          should_render.asset = !should_render.asset;
        },
        glfw::WindowEvent::Key(Key::Right, _,  Action::Press, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.right = true; 
        },
        glfw::WindowEvent::Key(Key::Right, _,  Action::Release, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.right = false; 
        },
        glfw::WindowEvent::Key(Key::Left, _,  Action::Press, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.left = true;
        },
        glfw::WindowEvent::Key(Key::Left, _,  Action::Release, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.left = false; 
        },
        glfw::WindowEvent::Key(Key::Up, _,  Action::Press, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.up = true; 
        },
        glfw::WindowEvent::Key(Key::Up, _,  Action::Release, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.up = false; 
        },
        glfw::WindowEvent::Key(Key::Down, _,  Action::Press, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.down = true;
        },
        glfw::WindowEvent::Key(Key::Down, _,  Action::Release, _) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.down = false; 
        },
        //this needs to use the scroll wheel not the O key
        glfw::WindowEvent::Scroll(_,y) => {
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.camera.zoom = Some(y as f32);
        },
        glfw::WindowEvent::Key(Key::D, _,  Action::Press, _) => {
          //Switch terrain type to passable
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.mode = Mode::Drawing(DrawMode::Dragging)
        },
        glfw::WindowEvent::Key(Key::D, _,  Action::Release, _) => {
          //Switch terrain type to passable

          //only add if a terrain is also set, 
          // then I can update the code that does drawing to check the drawing mode instead of terrain
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.mode = Mode::Drawing(DrawMode::Clicking)
        },
        glfw::WindowEvent::CursorPos(x,y) => {
          //Create the mouse's ray
          let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
          let transforms = world.immut_get_resource::<Transforms>().unwrap();
          let ray = MouseRay::new(x, y, &screen_dimensions, &transforms);
          
          //Add the ray to the input manager
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          
          if input_manager.mode == Mode::Drawing(DrawMode::Dragging) {
            input_manager.mouse.ray = Some(ray);
          }
        },
        glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
          let screen_dimensions = world.immut_get_resource::<ScreenDimensions>().unwrap();
          let transforms = world.immut_get_resource::<Transforms>().unwrap();
          let (x,y) = window.get_cursor_pos();
          let ray = MouseRay::new(x, y, &screen_dimensions, &transforms);
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          
          input_manager.mouse.ray = Some(ray);
        },
        glfw::WindowEvent::Key(Key::P, _,  Action::Press, _) => {
          //Switch terrain type to passable
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.grid.setting_terrain = Some(Terrain::Passable)
        },
        glfw::WindowEvent::Key(Key::I, _,  Action::Press, _) => {
          //Switch terrain type to impassable
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.grid.setting_terrain = Some(Terrain::Impassable)
        },
        glfw::WindowEvent::Key(Key::B, _,  Action::Press, _) => {
          //Switch terrain type to bush
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.grid.setting_terrain = Some(Terrain::Bush)
        },
        glfw::WindowEvent::Key(Key::N, _,  Action::Press, _) => {
          //Switch terrain type to none
          let input_manager = world.mut_get_resource::<InputManager>().unwrap();
          input_manager.grid.setting_terrain = None
        },
        glfw::WindowEvent::Key(Key::O, _,  Action::Press, _) => {
          //Get the file path
          //I think the pathbuf needs to also save the path to a resource or something so other systems can use it
          let pathbuf = open_file_dialog().unwrap();
          
          //Load the asset into the world and map it
          create_asset(pathbuf, &mut world).unwrap();
        }
        _ => {}
      }
    }

    //Render
    input(&mut world).unwrap();
    render(&world).unwrap();
    
    window.swap_buffers();
    thread::sleep(Duration::from_secs(1/30))
  }
}

//add file browser
// dim the colors a bit because they're too bright
//on scroll set camera target to where the mouse is pointing
// add saving
//figure out how to deal with the fact bush is a type of passable
// 5) add the ability to load from a file instead of having to hard code the file path
// 6) add saving
// 7) Add the ability to map different types of terrain instead of just "passable"/"impassable" 
// by checking the material associated with a vertex 
//add file system access
//add saving
//set up hot reloading
//ensure interoperability with the game
//add GUI???
// make a terrain selector