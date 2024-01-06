use crate::render::Vertex;

//try to minimize the use of nonserializable structs like Vec3
#[derive(Debug, Clone, Copy)]
pub struct Point{
  pub x:f32,
  pub y:f32,
  pub z:f32 
}

impl From<Vertex> for Point{
  fn from(value: Vertex) -> Self {
    Self{
      x:value.pos[0],
      y:value.pos[1],
      z:value.pos[2],
    }
  }
}

impl Point {
  pub fn new(x:f32, y:f32, z:f32 ) -> Self {
    Self { x, y, z }
  }
}