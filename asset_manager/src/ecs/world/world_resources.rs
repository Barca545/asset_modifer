use gl::{Gl,FRAGMENT_SHADER};

use crate::{ecs::World,render::Program, math::Transforms};

use eyre::Result;

#[derive(Debug, Clone, Copy)]
pub struct ScreenDimensions {
  pub width:i32,
  pub height:i32,
  pub aspect:f32,
}

impl ScreenDimensions {
  pub fn new(width:i32, height:i32) -> Self {
    let aspect = width as f32 / height as f32;
    ScreenDimensions { 
      height, 
      width, 
      aspect,
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Selected {
  NONE,
  HOVERED(usize),
  CLICKED(usize)
}

#[derive(Debug, Clone, Copy)]
pub enum ProgramType{
  Asset,
  Grid 
}

pub struct ShaderPrograms{
  pub asset: Program,
  pub grid: Program,
}

impl ShaderPrograms {
  pub fn new(world:&World) -> Result<Self> {
    let gl = world.immut_get_resource::<Gl>().unwrap();

    let mut asset = Program::new(&gl, "textured", "textured", FRAGMENT_SHADER).unwrap();

    asset
      .with_model(gl)?
      .with_view(gl)?
      .with_projection(gl)?;

    let mut grid = Program::new(&gl, "grid", "grid", FRAGMENT_SHADER).unwrap();

    grid
      .with_model(gl)?
      .with_view(gl)?
      .with_projection(gl)?;

    Ok(Self { 
      asset, 
      grid
    })
  }
  
  pub fn set_uniforms(&self, world:&World, program_type:ProgramType) {
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    let gl = world.immut_get_resource::<Gl>().unwrap();
    
    let program;

    match program_type {
      ProgramType::Asset => program = self.asset,
      ProgramType::Grid => program = self.grid,
    }

    program.use_program(gl);

    //Set the view transform's value
    program.set_view_matrix(gl, &transforms.view_transform);
    
    //Set the projection transform's value
    program.set_projection_matrix(gl, transforms.projection_transform.as_matrix());
  }

  // pub fn set_model_matrix()
}


pub struct ShouldRender{
  pub asset:bool,
  pub grid:bool
}

impl Default for ShouldRender{
  fn default() -> Self {
    Self::new(true, true)
  }
}

impl ShouldRender {
  pub fn new(asset:bool, grid:bool) -> Self {
    Self{
      asset,
      grid
    }
  }
}