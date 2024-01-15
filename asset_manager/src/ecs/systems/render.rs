use eyre::Result;

use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT};

use crate::{ecs::{World, world_resources::{ShaderPrograms, ShouldRender, ProgramType}, component_lib::{Asset, OutlineMesh, Position, NormalMesh}}, render::{render_mesh, render_lines, GridMesh, render_cells}, math::{Mat4, calculate_model_transform}, polygons::{Grid}};

pub fn render(world:&World) -> Result<()>{
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let programs = world.immut_get_resource::<ShaderPrograms>().unwrap();
  let should_render = world.immut_get_resource::<ShouldRender>().unwrap();

  //Clear the buffers
  unsafe {gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT)}
  render_grid(world)?;

  let mut query = world.query();

  let entities = query.with_component::<Asset>()?.run_entity();
  
  for entity in entities { 
    
    if should_render.asset{
      //Set the uniforms
      programs.set_uniforms(world, ProgramType::Asset); 
      
      let mesh = entity.immut_get_component::<NormalMesh>()?;
      let position = entity.immut_get_component::<Position>()?;

      //Get and set the model transform
      let model_transform:Mat4 = calculate_model_transform(&position.0, 1.0);
      programs.asset.set_model_matrix(gl, &model_transform);
      
      //Bind the shader programs
      programs.asset.use_program(gl);
      render_mesh(&gl, &mesh.0);
      programs.asset.unbind(gl);
    }
  }
  Ok(())
}

//need a new render function to render the grid and color in the shapes properly
pub fn render_grid(world:&World) -> Result<()>{
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let programs = world.immut_get_resource::<ShaderPrograms>().unwrap();

  //Set the uniforms
  programs.set_uniforms(world, ProgramType::Grid); 

  let mut query = world.query();

  let entities = query.with_component::<Grid>()?.run_entity();

  //Should only be one grid
  for entity in entities {
    let grid_mesh = entity.immut_get_component::<GridMesh>()?;
    //to keep it consistent maybe just hardcode the position here
    let position = entity.immut_get_component::<Position>()?;

    //Get and set the model transform
    let model_transform:Mat4 = calculate_model_transform(&position.0, 1.0);
    programs.grid.set_model_matrix(gl, &model_transform);
    
    //Bind the shader programs
    programs.grid.use_program(gl);
    //Render the cells
    render_cells(&gl, &grid_mesh);

    //Render the lines
    //figure out why the cells are overlapping them
    render_lines(&gl, &grid_mesh);
    programs.grid.unbind(gl);
  }
  
  Ok(())
}

//unsure if these should be their own function or not
pub fn render_asset(){}
pub fn render_outline(){}