use eyre::Result;
use gl::Gl;

use crate::{ecs::World, input_manager::InputManager, math::{Transforms, Vec3}, polygons::{Grid, Terrain}, render::GridMesh};

pub fn input(world:&mut World) -> Result<()>{
  camera_input(world)?;
  mouse_input(world)?;
  grid_input(world)?;
  update_grid_cells(world)?;
  Ok(())
}

fn camera_input(world:&mut World) -> Result<()> {
  //Create a new camera by copying the previous one
  //unsure I need to clone here, might just be able to deref it and change it directly
  let mut camera = world.immut_get_resource::<Transforms>().unwrap().camera.clone();
  let input_manager = world.mut_get_resource::<InputManager>().unwrap();
  let camera_speed = 0.001;
  let scroll_speed = 0.6;

  //Update the camera position 
  if input_manager.camera.right {
    camera.position += Vec3::new(-1.0,0.0,0.0) * camera_speed;
    camera.target += Vec3::new(-1.0,0.0,0.0) * camera_speed;
  }
  if input_manager.camera.left {
    camera.position += Vec3::new(1.0,0.0,0.0) * camera_speed;
    camera.target += Vec3::new(1.0,0.0,0.0) * camera_speed;
  }
  if input_manager.camera.up {
    camera.position += Vec3::new(0.0,0.0,1.0) * camera_speed;
    camera.target += Vec3::new(0.0,0.0,1.0) * camera_speed;
  }
  if input_manager.camera.down {
    camera.position += Vec3::new(0.0,0.0,-1.0) * camera_speed;
    camera.target += Vec3::new(0.0,0.0,-1.0) * camera_speed;
  }
  match input_manager.camera.zoom{
    None => {},
    Some(y) => {
      camera.position += y * Vec3::new(0.0,-1.0,0.0) * scroll_speed;
      //also need to delete the scroll amount
    },
  }
  input_manager.camera.zoom = None;
  
  //Recalculate the view matrix
  let transforms = world.mut_get_resource::<Transforms>().unwrap();
  transforms.update_view(&camera);

  Ok(())
}

fn mouse_input(world:&mut World) -> Result<()> {
  let input_manager = world.mut_get_resource::<InputManager>().unwrap();

  //this is actually where the logic for checking if a terrain is set/if it should update the grid  should go not in the grid input

  Ok(())
}

fn grid_input(world:&mut World) -> Result<()> {
  let input_manager = world.immut_get_resource::<InputManager>().unwrap();

  //Check if a terrain is set
  if input_manager.grid.setting_terrain != None {
    //Get the position of the mouse on the grid and update the targeted index in the grid inputs
    if let Some(ray) = input_manager.mouse.ray {
      let mouse_position = ray.0.ray_ground_intersection();
      let index;
      {
        //Get the selected index
        let mut query = world.query();
  
        let entities = query.with_component::<Grid>()?.run_entity();
  
        let entity = &entities[0];
        let grid = entity.immut_get_component::<Grid>()?;
        index = grid.get_cell_index(mouse_position);
      }
        
      //Update the input manager with the targeted index
      let input_manager = world.mut_get_resource::<InputManager>().unwrap();
      
      //Add the new target cell to the list of targeted cells
      input_manager.grid.target_cells.push(index);
  
      //Reset the mouse ray
      input_manager.mouse.ray = None;
    }
  }

  else {
    //Reset the mouse ray
    let input_manager = world.mut_get_resource::<InputManager>().unwrap();
    input_manager.mouse.ray = None;
  }
  
  Ok(())
}

//this will not stay in the input, this is just for a test
fn update_grid_cells(world:&mut World) -> Result<()> {
  let gl = world.immut_get_resource::<Gl>().unwrap().clone();
  let grid_input = world.mut_get_resource::<InputManager>().unwrap();

  // let indices = world.mut_get_resource::<InputManager>().unwrap().grid.target_cells.clone();
  // dbg!(indices);

  //terrain might just need a none option tbh
  let mut setting_terrain = Terrain::Passable;
  let mut indices = Vec::default();

  //If there is a terrain set for altering cells, continue
  if let Some(terrain) = grid_input.grid.setting_terrain{
    setting_terrain = terrain;

    // Iterate over the cached indices and update the grid location they describe with the new terrain
    while let Some(index) = grid_input.grid.target_cells.pop(){
      indices.push(index);
    }
  }

  for index in indices {
    let mut query = world.query();
  
    let entities = query.with_component::<Grid>()?.run_entity();
    
    let entity = &entities[0];
    let mut grid = entity.mut_get_component::<Grid>()?;
    let mut grid_mesh = entity.mut_get_component::<GridMesh>()?;

    match setting_terrain {
      Terrain::Passable => {
        //should be blue
        grid.cells[index] = Terrain::Passable;
        grid_mesh.color_cell(&gl, index,[0.0,0.0,1.0]);
      },
      Terrain::Impassable => {
        grid.cells[index] = Terrain::Impassable;
        grid_mesh.color_cell(&gl, index,[1.0,0.0,0.0]);
      },    
      Terrain::Bush => {
        //should be green
        grid.cells[index] = Terrain::Bush;
        grid_mesh.color_cell(&gl, index,[0.0,1.0,0.0]);
      },
      
    }   
  }
  Ok(())
}