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

pub struct ShaderProgram(pub Program); 

impl ShaderProgram {
  pub fn new(world:&World) -> Result<Self> {
    let gl = world.immut_get_resource::<Gl>().unwrap();

    //might need new shaders
    let mut normal = Program::new(&gl, "textured", "textured", FRAGMENT_SHADER).unwrap();
    
    normal
      .with_model(gl)?
      .with_view(gl)?
      .with_projection(gl)?;

    Ok(Self(normal))
  }
  
  pub fn set_uniforms(&self, world:&World) {
    let transforms = world.immut_get_resource::<Transforms>().unwrap();
    let gl = world.immut_get_resource::<Gl>().unwrap();
    let program = self.0;

    program.use_program(gl);

    //Set the view transform's value
    program.set_view_matrix(gl, &transforms.view_transform);
    
    //Set the projection transform's value
    program.set_projection_matrix(gl, transforms.projection_transform.as_matrix());
  }
}


pub struct ShouldRender{
  pub asset:bool,
  pub outline:bool
}

impl Default for ShouldRender{
  fn default() -> Self {
    Self{
      asset:false,
      outline:true
    }
  }
}

impl ShouldRender {
  pub fn new(asset:bool, outline:bool) -> Self {
    Self{
      asset,
      outline
    }
  }
}