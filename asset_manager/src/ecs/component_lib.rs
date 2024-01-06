use gl::Gl;
use serde::{Deserialize, Serialize};

use crate::{
  math::Vec3, render::{Mesh, Vertex},
  //unsure if I need this for selection since top down
  // physics::AABB3D,
  };

//I think I want to separate these into two components
#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub Vec3);

impl Position {
  pub fn new(x:f32,y:f32,z:f32) -> Self {
    Position(Vec3::new(x,y,z))
  }
}

// #[derive(Debug, Clone, Copy)]
// ///3D AABB to be used for unit selection.
// pub struct SelectionRadius(pub AABB3D);

// impl SelectionRadius {
//   pub fn new(position:Vec3, height:f32, radius:f32) -> Self {
//     let aabb3d = AABB3D::new(position, height, radius);

//     SelectionRadius(aabb3d)
//   }
// }

pub struct Asset;
pub struct MeshPoint;

//rendering
pub struct NormalMesh(pub Mesh);

impl NormalMesh{
  pub fn new(gl:&Gl, vertices:Vec<Vertex>, indices:Vec<u32>, texture_name:&str) -> Self{
    NormalMesh(Mesh::new(gl, vertices, indices, texture_name))
  }
}

pub struct OutlineMesh(pub Mesh);

impl OutlineMesh{
  pub fn new(gl:&Gl, vertices:Vec<Vertex>, indices:Vec<u32>, texture_name:&str) -> Self{
    OutlineMesh(Mesh::new(gl, vertices, indices, texture_name))
  }
}

