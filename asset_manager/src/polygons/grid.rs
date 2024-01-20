use std::collections::HashMap;

use gl::Gl;

use crate::{render::{Vertex, GridMesh}, math::Vec3};

use eyre::Result;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Terrain{
  #[default]
  Passable,
  Impassable,
  Bush,
}

/// The grid begins with the top left cell and is zero indexed.
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
  /// Grid starts in the bottom right corner.
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
    let mut column_index = (adjusted_x/self.cell_size).floor() as i32 + self.column_offset;
    let mut row_index = (adjusted_z/self.cell_size).floor() as i32 + self.row_offset;

    //If the index is greater than the max value set it to the max value
    //If the index is smaller than 0 value set it to 0
    if column_index >= self.num_columns as i32{
      column_index = self.num_columns as i32 -1;
    }
    if column_index < 0 {
      column_index = 0;
    }
    if row_index >= self.num_rows as i32{
      row_index = self.num_rows as i32 -1;
    }
    if row_index < 0 {
      row_index = 0;
    }

    //Calculate the cell index
    let cell_index = (row_index as usize * self.num_columns) + column_index as usize;
    cell_index
  }
}

//this isn't generating a grid from the object
//this might need to be an external function
//it's updating the inside of the cells to indicate if the type of terrain in the cell
//maybe make a struct to hold vertices and indices instead of having to pass them into a tupl
pub fn map_object(gl:&Gl, object_vertices:&Vec<Vertex>, object_indices:&Vec<u32>, grid:&mut Grid, grid_mesh:&mut GridMesh){ 
  let mut overlapped_indices: HashMap<usize, Terrain> = HashMap::new();
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
    

    //Check the mtl info to see the material type
    //Color the vertex: 
    // - GREEN for passable
    // - RED for impassable
    let mtl = Terrain::Impassable;
    
    match mtl {
      Terrain::Passable => {
        overlapped_indices.insert(index, Terrain::Passable);
      },
      Terrain::Impassable => {
        overlapped_indices.insert(index, Terrain::Impassable);
      },
      Terrain::Bush => {
        overlapped_indices.insert(index, Terrain::Bush);
      }
    }
  }
  //Update the cell terrain info, mesh and color 
  for (index, terrain) in overlapped_indices {
    match terrain {
      Terrain::Passable => {
        // Turn the cell blue
        grid.cells[index] = Terrain::Impassable;
        grid_mesh.color_cell(gl, index,[0.0,0.0,1.0]);
      },
      Terrain::Impassable => {
        // Turn the cell red
        grid.cells[index] = Terrain::Impassable;
        grid_mesh.color_cell(gl, index,[1.0,0.0,0.0]);
      },
      
      Terrain::Bush => {
        //Turn the cell green
        grid.cells[index] = Terrain::Impassable;
        grid_mesh.color_cell(gl, index,[0.0,1.0,0.0]);
      }
    }
  }
}