mod create;
mod render;
mod polygons;
mod math;
mod errors;
mod filesystem;
mod ecs;

use std::{thread, time::Duration};
use create::create_gl;
use ecs::{World, component_lib::{NormalMesh, OutlineMesh, Asset, Position}, systems::render, world_resources::{ShouldRender, ShaderProgram, ScreenDimensions}};
use filesystem::{load_object, load_object_outline};
use glfw::{Key, Action, Context, MouseButton};
use math::Transforms;
use polygons::{create_complex_collider, Grid};
use render::GridMesh;
use crate::{create::create_window, ecs::component_lib::MeshPoint};

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

  let program = ShaderProgram::new(&world).unwrap();
  
  world  
    .add_resource(gl.clone())
    .add_resource(dragging)
    .add_resource(ShouldRender::default())
    .add_resource(program);

  world
    .register_component::<Asset>()
    .register_component::<MeshPoint>()
    .register_component::<NormalMesh>()
    .register_component::<OutlineMesh>()
    .register_component::<Position>()
    .register_component::<Grid>()
    .register_component::<GridMesh>();

  //Load the vertices and indices
  //this is where I need to handle the result by displaying it to the user
  let name = "ball";
  let (asset_vertices, asset_indices) = load_object(name).unwrap();
  // let (outline_vertices, outline_indices) = load_object_outline((asset_vertices.clone(), asset_indices.clone())).unwrap();
  let (outline_vertices, outline_indices) = load_object_outline(name).unwrap();
  //possibly make texture name an option or something *or* just make the outline have its own shader  
  let asset_mesh = NormalMesh::new(&gl, asset_vertices, asset_indices, "blank_texture");
  let outline_mesh = OutlineMesh::new(&gl, outline_vertices.clone(), outline_indices.clone(), "red");
  // let outline_voxels = voxelize_obj(&outline_vertices, &outline_indices, 3);
  // let collider = create_complex_collider(outline_vertices, &outline_indices);
  // dbg!(collider);
  // let (asset_vertices, asset_indices) = load_object("ball-polyline").unwrap();
  // dbg!(asset_vertices);

  // let mut test = 0.0;
  // for voxel in outline_voxels.voxels() { 
  //  let (outline_vertices, outline_indices) = load_object_outline("ball-polyline").unwrap();
  //  let voxel_point_mesh = OutlineMesh::new(&gl, outline_vertices, outline_indices, "blank_texture");
  
  //  let position = voxel.coords;
  //  dbg!(position);
  //  world
  //   .create_entity()
  //   .with_component(MeshPoint).unwrap()
  //   .with_component(Asset).unwrap()
  //   .with_component(voxel_point_mesh).unwrap()
  //   .with_component(Position::new(position.x as f32,0.0,position.y as f32)).unwrap();
  // }

  

  world
    .create_entity()
    .with_component(Asset).unwrap()
    .with_component(asset_mesh).unwrap()
    .with_component(outline_mesh).unwrap()
    .with_component(Position::new(0.0,0.0,0.0)).unwrap();

  //create the grid 
  let grid = Grid::new(10, 10, 1.0);
  let grid_mesh = GridMesh::new(&gl, &grid);
  
  world
    .create_entity()
    .with_component(grid).unwrap()
    .with_component(grid_mesh).unwrap();
  
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
  // it occurs to me 
  // 1) 
  // 2)


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