// use parry2d::{math::Point, transformation::voxelization::{VoxelSet, FillMode}};
use polygon_clipping::Polygon;

use crate::{render::Vertex, math::Point};

// //create a convex decomposition of the mesh into voxel data
// //voxels can be used for collisions
// pub fn voxelize_obj(vertices:&Vec<Vertex>, indices:&Vec<u32>, resolution:u32) -> VoxelSet {
//   //convert the vertices to Points
//   let indices = indices.clone();
//   let mut points = Vec::default();
//   let mut index_buffers = Vec::default();
  
//   for index in indices {
//     // let index = index ;
//     let vertex = vertices[index as usize];
//     let x = vertex.pos[0];
//     let z = vertex.pos[2];
//     let point = Point::new(x,z);
//     points.push(point);
//     let index_buffer = [index, index];
//     index_buffers.push(index_buffer);
//   }
  
//   let fill_mode = FillMode::SurfaceOnly;
//   let voxels = VoxelSet::voxelize(&points, &index_buffers, resolution, fill_mode, false);
//   voxels
// }

//not sure what these points are calculated in relation to
#[derive(Debug, Clone, Copy)]
pub struct AABB2D {
  pub min: Point,
  pub max: Point
}

impl AABB2D { 
  ///Returns true if the aabb is inside `other` or if they are the same box.
  fn is_inside(&self, other:&AABB2D) -> bool {
    //checks if self min is > than other min
    self.min.x >= other.min.x && self.min.y >= other.min.y
    &&
    //checks if self max is < than other max
    self.max.x <= other.max.x && self.max.y <= other.max.y
  }
}

impl From<&Vec<Vertex>> for AABB2D {
  fn from(value: &Vec<Vertex>) -> Self {
    
    let mut min_x = 0.0;
    let mut max_x = 0.0;
    let mut min_z = 0.0; 
    let mut max_z = 0.0;
    
    for vertex in value {
      //Check against the the min_x value and replace if x is smaller
      let x = vertex.pos[0];
      let z = vertex.pos[2];
      if x < min_x{
        min_x = x
      }
      //Check against the the max_x value and replace if x is bigger
      if x > max_x{
        max_x = x
      }
      //Check against the the min_z value and replace if z is smaller
      if z < min_z{
        min_z = z
      }
      //Check against the the max_z value and replace if z is bigger
      if z > max_z{
        max_z = z
      }
    }

    let min = Point::new(min_x,0.0,min_z);
    let max = Point::new(max_x,0.0,max_z);
    Self { min, max}
  }

  
}

///Given a `Mesh`'s vertices and indices, returns a `Vec<AABB2D>` containing the bounding boxes that outline the shape.
pub fn create_complex_collider(vertices:Vec<Vertex>, indices:&Vec<u32>) -> Vec<AABB2D>{
  let mut boxes:Vec<AABB2D> = Vec::new();
  let mut vertex_holder = Vec::new();
  
  dbg!(indices.len());

  //Loop through all the indices and add the vertices they correspond to to the vertex_holder
  for index in indices {
    //If there are 3 vertices in the vertex_holder, create a bounding 2dAABB
    if vertex_holder.len() == 4 {
      // let vertex_1 = vertex_holder.pop().unwrap();
      // let vertex_2 = vertex_holder.pop().unwrap();
      // let vertex_3 = vertex_holder.pop().unwrap();
      // let vertex_4 = vertex_holder.pop().unwrap();
      
      // //Find the square that contains all three points
      // let vertices = [vertex_1,vertex_2,vertex_3];
      let aabb = AABB2D::from(&vertex_holder);
      boxes.push(aabb)
    }

    //Add the next vertex to the buffer
    let vertex = vertices[*index as usize];
    vertex_holder.push(vertex);
  }
  
  
  //Once all of the AABBs have been created, delete all of the ones inside another one
  let mut final_boxes:Vec<AABB2D> = Vec::new();
  
  for (index,aabb) in boxes.iter().enumerate() { 
    //check the box is not inside any other box in boxes
    let mut is_inside = false;
    
    for other in boxes.iter().skip(index){
      is_inside = aabb.is_inside(other);
    }
    
    if is_inside == false {
      //make sure this is copying and not moving the value
      final_boxes.push(*aabb)
    }
  }

  final_boxes
}


fn delete_edges(){}