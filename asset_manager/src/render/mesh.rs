use gl::{Gl, STATIC_DRAW};

use crate::polygons::Grid;

use super::{Vertex, buffer::{VertexArray, ArrayBuffer, ElementArrayBuffer}, texture::Texture};

#[derive(Debug, Clone)]
pub struct Mesh {
  pub vertices:Vec<Vertex>,
  pub indices:Vec<u32>,
  pub texture:Texture,
  pub vao:VertexArray,
  pub vbo:ArrayBuffer,
  pub ebo:ElementArrayBuffer
}

impl Mesh {
  pub fn new(gl:&Gl, vertices:Vec<Vertex>, indices:Vec<u32>, texture_name:&str) -> Self {
    let texture = Texture::new(gl, texture_name).unwrap();
    let (vao, vbo, ebo) = Self::init_mesh(gl, &vertices, &indices);
  
    Mesh {vertices, indices, texture, vao, vbo, ebo }
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<Vertex>, indices:&Vec<u32>,) -> (VertexArray, ArrayBuffer, ElementArrayBuffer) {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);
    let ebo = ElementArrayBuffer::new(&gl);

    //buffer the vertex information to the vao
    vao.bind(gl); 
    
    vbo.bind(gl);
    vbo.buffer_data(gl, &vertices, STATIC_DRAW);
    
    ebo.bind(gl);
    ebo.buffer_data(gl,&indices, STATIC_DRAW);
    
    Vertex::init_attrib_pointers(&gl);
    
    vbo.unbind(gl);
    ebo.unbind(gl);
    
    vao.unbind(gl);
    
    (vao,vbo,ebo)
  }
}

//ok what I want to do is draw the grid lines which can be done using the prior method and then shade them in
//this grid mesh struct should hold a cell mesh and a line mesh
//render the lines over (after) the grid
//might need to change the line color
pub struct GridMesh{
  pub cells:CellMesh,
  pub lines:LineMesh
}

impl GridMesh {
  pub fn new(gl:&Gl, grid:&Grid) -> Self {
    let cells = CellMesh::new(gl, grid);
    let lines = LineMesh::new(gl, grid);

    GridMesh { 
      cells, 
      lines
    }
  }

  pub fn color_cell(&mut self, gl:&Gl, index:usize, color:[f32;3]){
    self.cells.color_cell(gl, index, color)
  }
}

pub struct CellMesh{
  pub vertices:Vec<Vertex>,
  pub vao:VertexArray,
  pub vbo:ArrayBuffer,
}

impl CellMesh {
  pub fn new(gl:&Gl, grid:&Grid) -> Self{
    let cell_size = grid.cell_size;
    let height_bound = (grid.num_rows/2) as i32;
    let width_bound = (grid.num_columns/2) as i32;

    let mut vertices = Vec::default();

    for i in (-height_bound..height_bound).rev() {
      for j in (-width_bound..width_bound).rev() {
        //Convert the coordinates into floats
        let x = j as f32 * cell_size;
        let y = 0.0;
        let z = i as f32 * cell_size;
        let color = [0.5,0.5,0.5];

        let vertex_1= Vertex::from((x, y, z)).color(color);
        let vertex_2= Vertex::from((x+cell_size, y, z)).color(color);
        let vertex_3= Vertex::from((x+cell_size, y, z+cell_size)).color(color);
        let vertex_4= Vertex::from((x+cell_size, y, z+cell_size)).color(color);
        let vertex_5= Vertex::from((x, y, z+cell_size)).color(color);
        let vertex_6= Vertex::from((x, y, z)).color(color);

        //Add the new verts to the vertices vector
        let new_cell = vec![
          vertex_1, 
          vertex_2, 
          vertex_3, 
          vertex_4, 
          vertex_5, 
          vertex_6, 
        ];
        vertices.extend(new_cell);
      }
    }

    let (vao, vbo) = Self::init_mesh(gl, &vertices);

    CellMesh{
      vao,
      vbo,
      vertices
    }    
  }

  pub fn color_cell(&mut self, gl:&Gl, index:usize, color:[f32;3]){
    //Iterate over the 8 vertices of a cell and color appropriately

    //currently this is not inclusive because for some reason the last index panics
    for offset in 0..6 {
      //Converts the index into the proper starting point. 
      //For example, the 3rd cell starts at slice 24 because each cell is 8 vertices long
      let vertex_index = 6*index + offset;
      self.vertices[vertex_index].clr = color;
    }

    //Update the mesh's vbo with the recolored vertices
    self.update_vertices(gl, &self.vertices)
  }

  fn update_vertices(&self, gl:&Gl, vertices:&Vec<Vertex>){
    self.vao.bind(gl); 
    self.vbo.bind(gl);
    self.vbo.buffer_data(gl, &vertices, STATIC_DRAW);
    Vertex::init_attrib_pointers(&gl);
    self.vbo.unbind(gl);
    self.vao.unbind(gl);
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<Vertex>) -> (VertexArray, ArrayBuffer) {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);

    //Buffer the vertex information to the vao
    vao.bind(gl); 
    vbo.bind(gl);
    vbo.buffer_data(gl, &vertices, STATIC_DRAW);
    Vertex::init_attrib_pointers(&gl);
    vbo.unbind(gl);
    vao.unbind(gl);
    
    //Return the vao and vbo
    (vao,vbo)
  }
}

pub struct LineMesh{
  pub vertices:Vec<Vertex>,
  pub vao:VertexArray,
  pub vbo:ArrayBuffer,
}

impl LineMesh {
  pub fn new(gl:&Gl, grid:&Grid) -> Self{
    let cell_size = grid.cell_size;
    let height_bound = (grid.num_rows/2) as i32;
    let width_bound = (grid.num_columns/2) as i32;

    let mut vertices = Vec::default();

    for i in (-height_bound..height_bound).rev() {
      for j in (-width_bound..width_bound).rev() {
        //Convert the coordinates into floats
        let x = j as f32 * cell_size;
        //render it *slightly* higher than the cells so it is on top
        let y = 0.001;
        let z = i as f32 * cell_size;
        let color = [1.0,0.0,0.0];

        //Create the 8 vertices of a grid square.
        let vertex_1= Vertex::from((x, y, z)).color(color);
        let vertex_2= Vertex::from((x+cell_size, y, z)).color(color);

        let vertex_3= Vertex::from((x+cell_size, y, z)).color(color);
        let vertex_4= Vertex::from((x+cell_size, y, z+cell_size)).color(color);
        
        let vertex_5= Vertex::from((x+cell_size, y, z+cell_size)).color(color);
        let vertex_6= Vertex::from((x, y, z+cell_size)).color(color);

        let vertex_7= Vertex::from((x, y, z+cell_size)).color(color);
        let vertex_8= Vertex::from((x, y, z)).color(color);

        let new_cell = vec![
          vertex_1, 
          vertex_2, 
          vertex_3, 
          vertex_4, 
          vertex_5, 
          vertex_6, 
          vertex_7, 
          vertex_8
        ];
        vertices.extend(new_cell);
      }
    }

    let (vao, vbo) = Self::init_mesh(gl, &vertices);

    LineMesh{
      vao,
      vbo,
      vertices
    }    
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<Vertex>) -> (VertexArray, ArrayBuffer) {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);

    //Buffer the vertex information to the vao
    vao.bind(gl); 
    vbo.bind(gl);
    vbo.buffer_data(gl, &vertices, STATIC_DRAW);
    Vertex::init_attrib_pointers(&gl);
    vbo.unbind(gl);
    vao.unbind(gl);
    
    //Return the vao and vbo
    (vao,vbo)
  }
}