use std::collections::HashMap;

use gl::Gl;

use crate::{render::{Vertex, GridMesh}, math::Vec3};

use eyre::Result;

struct Cell{
  width: u32,
  height: u32
}

#[derive(Debug, Clone, Copy)]
pub enum Terrain{
  Passable,
  Impassable,
  Bush
}

///Cell size is both the height and width of a cell since the cells are squares.
pub struct Grid{
  pub num_columns: usize,
  pub num_rows: usize,
  pub cell_size: f32,
  //unsure if the offsets should be pub 
  column_offset: i32,
  row_offset: i32,
  pub cells:Vec<Terrain>
}

impl Grid {
  ///Creates a new grid.
  pub fn new(width:u32, height:u32, cell_size:f32) -> Result<Self> {
    //Calculate the number of columns/rows 
    let num_columns = ((width as f32)/cell_size) as usize;
    let num_rows = ((height as f32)/cell_size) as usize;
    
    //Check to ensure the grid is divisible by two and the cell size is greater than 0
    assert!(num_columns % 2 == 0);
    assert!(num_rows % 2 == 0);
    assert!(cell_size > 0.0);

    //Calculate the offsets of the grid by dividing the number of rows and columns since have the grid is negative
    let column_offset = (num_columns/2) as i32;
    let row_offset = ((num_rows+1)/2) as i32;
    
    //Create a vector of Terrain initialized to passable containing exactly as many cells as the grid 
    let max_index = num_columns * num_rows;
    let cells = vec![Terrain::Passable; max_index];
    
    Ok(Self{
      num_columns,
      num_rows,
      cell_size,
      column_offset,
      row_offset,
      cells
    })
  }

  ///Returns the index of a cell from a given position.
  /// The process the function uses is called a raster scan.
  /// Uses floor to calculate the x_index and z_index meaning it defaults towards the bottom-left square if on a cell boundy.
  pub fn get_cell_index(&self, position:Vec3) -> usize {
    //Offset the position values since the grid is centered at (0,0) instead of starting there
    let adjusted_x = position.x;
    let adjusted_z = position.z;

    //Calculate the row and column indices
    let mut column_index = ((adjusted_x/self.cell_size).floor() as i32 + self.column_offset) as usize;
    let mut row_index = ((adjusted_z/self.cell_size).floor() as i32 + self.row_offset) as usize;

    //If the index overflows, set it to the maximum value
    if column_index >= self.num_columns{
      column_index = self.num_columns-1;
    }
    if row_index >= self.num_rows{
      row_index = self.num_rows-1;
    }

    //Calculate the cell index
    let cell_index = (row_index * self.num_columns) + column_index;
    cell_index
  }
}

//this isn't generating a grid from the object
//this might need to be an external function
//it's updating the inside of the cells to indicate if the type of terrain in the cell
//maybe make a struct to hold vertices and indices instead of having to pass them into a tupl
pub fn map_object(gl:&Gl, object_vertices:&Vec<Vertex>, object_indices:&Vec<u32>, grid:&mut Grid, grid_mesh:&mut GridMesh){ 
  let mut overlapped_indices = HashMap::new();
  //Extract the vertex coordinates 
  for index in object_indices{
    let vertex = object_vertices[*index as usize];
    
    //Get the vertex's position values and convert them into a Vec3
    let vert_x = vertex.pos[0];
    let vert_y = vertex.pos[1];
    let vert_z = vertex.pos[2];
    let position = Vec3::new(vert_x, vert_y, vert_z);

    //Get the index of the cell containing the vertex
    let index = grid.get_cell_index(position);
    overlapped_indices.insert(index, index);

    //Check the mtl info to see the material type
    //Color the vertex: 
    // - GREEN for passable
    // - RED for impassable
    let mtl = Terrain::Impassable;
    
    match mtl {
      Terrain::Passable => {
        //should be blue
      },
      Terrain::Impassable => {
        grid.cells[index] = Terrain::Impassable;
        //something is going wrong in the color cell code causing the targeted cell to be shifted down 1 right 1
        grid_mesh.color_cell(gl, index,[1.0,0.0,0.0]);
      },
      
      Terrain::Bush => {
        //should be green
      }
    }
  }
  // dbg!(overlapped_indices);
}

#[cfg(test)]
mod tests{
  use crate::{polygons::Grid, math::Vec3};

  #[test]
  fn get_index(){
    let grid = Grid::new(2, 2, 1.0).unwrap();
    
    // let test_position_1 = Vec3::new(1.0,0.0,-1.0);
    // let test_index_1 = grid.get_cell_index(test_position_1);
    // assert_eq!(1,test_index_1);

    let test_position_2 = Vec3::new(-1.0,0.0,0.0);
    let test_index_2 = grid.get_cell_index(test_position_2);
    assert_eq!(2,test_index_2);
  }

  #[test]
  fn get_index_big_grid(){
    let cell_size = 0.5;
    let grid = Grid::new(4, 4, cell_size).unwrap();
    
    let test_position = Vec3::new(-0.5,0.0,0.5);
    let test_index = grid.get_cell_index(test_position);
    
    if cell_size == 0.5{
      // assert_eq!(7,test_index)
    }

    if cell_size == 1.0 {
      // assert_eq!(6,test_index)
    }
    
    // assert_eq!(6,test_index);
  }

  #[test]
  fn update_mesh_colors(){}
}