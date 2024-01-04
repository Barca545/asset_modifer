use eyre::Result;

use gl::{Gl, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT};
use image::flat::NormalForm;

use crate::{ecs::{World, world_resources::{ShaderProgram, ShouldRender}, component_lib::{Asset, OutlineMesh, Position, NormalMesh}}, render::render_mesh, math::{Mat4, calculate_model_transform}};

pub fn render(world:&World) -> Result<()>{
  let gl = world.immut_get_resource::<Gl>().unwrap();
  let program = world.immut_get_resource::<ShaderProgram>().unwrap();
  let should_render = world.immut_get_resource::<ShouldRender>().unwrap();

  let mut query = world.query();

  let entities = query.with_component::<Asset>()?.run_entity();

  //Clear the buffers
  unsafe {gl.Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT)}

  //Set the uniforms
  program.set_uniforms(world); 

  for entity in entities { 
    
    if should_render.asset{
      let mesh = entity.immut_get_component::<NormalMesh>()?;
      let position = entity.immut_get_component::<Position>()?;

      //Get and set the model transform
      let model_transform:Mat4 = calculate_model_transform(&position.0, 1.0);
      program.0.set_model_matrix(gl, &model_transform);
      
      //Bind the shader programs
      program.0.use_program(gl);
      render_mesh(&gl, &mesh.0);
    }

    if should_render.outline{
      let mesh = entity.immut_get_component::<OutlineMesh>()?;
      let position = entity.immut_get_component::<Position>()?;

      //Get and set the model transform
      let model_transform:Mat4 = calculate_model_transform(&position.0, 1.0);
      program.0.set_model_matrix(gl, &model_transform);
      
      //Bind the shader programs
      program.0.use_program(gl);
      render_mesh(&gl, &mesh.0);
    }
  }

  Ok(())
}