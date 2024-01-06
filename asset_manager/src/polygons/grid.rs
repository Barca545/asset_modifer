use gl::Gl;

use crate::render::Vertex;

struct Cell{
  width: u32,
  height: u32
}

#[derive(Debug, Clone, Copy)]
pub enum Terrain{
  Impassable,
  Bush
}

pub struct Grid{
  pub width: usize,
  pub height: usize,
  pub cell_size: f32,
  pub cells:Vec<Vec<Terrain>>
}

impl Grid {
  pub fn new(width:usize, height:usize, cell_size:f32) -> Self {
    //cells are indexed by width then height
    let cells = vec![vec![]];
    Self{
      width,
      height,
      cell_size,
      cells
    }
  }

  //don't call this from object, maybe call this like map object or something, 
  //this isn't generating a grid from the object, 
  //it's updating the inside of the cells to indicate if the type of terrain in the cell
  pub fn map_object(&mut self, vertices:&Vec<Vertex>, indices:&Vec<usize>){
     //Load the object
     
     //Extract the vertex coordinates 
    for index in indices{
      let vertex = vertices[*index];
      let vert_x = vertex.pos[0];
      let vert_z = vertex.pos[2];
      
      //Check the mtl info to see the material type
      //Color the vertex accordingly GREEN for passable, RED for impassable
      let mtl = Terrain::Impassable;

      //Convert the vertex coords into a grid indices
      let horizontal_index = (vert_x/(self.width as f32)) as usize;
      let vertical_index = (vert_z/(self.height as f32)) as usize;

      self.cells[horizontal_index][vertical_index] = mtl;
    }
  }
}
