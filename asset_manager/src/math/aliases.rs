use nalgebra_glm::{
  TMat4, 
  TVec2, 
  TVec3, 
  TVec4,
  identity as identity_glm, 
  look_at as look_at_glm, 
  scale as scale_glm, 
  translate as translate_glm,
  inverse as inverse_glm
};

use nalgebra::Perspective3;

use super::camera::Camera;

pub type Vec2 = TVec2<f32>;
pub type Vec3 = TVec3<f32>;
pub type Vec4 = TVec4<f32>;
pub type Mat4 = TMat4<f32>;
pub type Perspective = Perspective3<f32>;

///Returns an `Mat4` identity matrix.
pub fn identity() -> Mat4{
  identity_glm::<f32, 4>()
}

///Builds a view matrix from a `Camera` struct.
pub fn look_at(camera:&Camera) -> Mat4 {
  look_at_glm(&camera.position, &camera.target, &camera.up)
}

///Translate a matrix to the location of position.
pub fn translate(matrix:&Mat4, position:&Vec3) -> Mat4 {
  translate_glm(matrix,position)
}

///Scales a matrix by a given scale factor.
pub fn scale(matrix:&Mat4, scale_factor:f32) -> Mat4 {
  scale_glm(matrix,&Vec3::new(scale_factor, scale_factor, scale_factor))
}

pub fn inverse(matrix:&Mat4) -> Mat4{
  inverse_glm(matrix)
}