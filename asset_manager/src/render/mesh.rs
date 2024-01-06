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

pub struct GridMesh{
  pub vertices:Vec<Vertex>,
  pub vao:VertexArray,
  pub vbo:ArrayBuffer,
}

//currently has four issues
//wrong scale
//shifted over
//needs new shader
//not a grid
impl GridMesh {
  ///Generates the vertices for needed to render the grid
  pub fn new(gl:&Gl, grid:&Grid) -> Self {
    let mut vertices = Vec::default();

    for i in 0..grid.width+1{
      for j in 0..grid.height+1 {
        // let x = (i as f32) * grid.cell_size;
        // let y = 0.0;
        // let z = (j as f32) * grid.cell_size;
        
        //according to GL this will result in the coordinates being from -1 to 1 in NDC
        let x = (i as f32) / (grid.width-1) as f32;
        let y = 0.0;
        let z = (j as f32) / (grid.height-1) as f32;

        // let x = (i as f32) / (grid.width) as f32;
        // let y = 0.0;
        // let z = (j as f32) / (grid.height) as f32;

        //I think I can give it random text coords since I won't render a texture onto it
        let vertex =  Vertex::from((x,y,z,0.0,0.0));
        vertices.push(vertex);
      }
    }

    let (vao, vbo) = Self::init_mesh(gl, &vertices);

    GridMesh{
      vertices,
      vao,
      vbo
    }
  }

  fn init_mesh(gl:&Gl, vertices:&Vec<Vertex>) -> (VertexArray, ArrayBuffer) {
    let vao = VertexArray::new(&gl);
    let vbo = ArrayBuffer::new(&gl);
    // let ebo = ElementArrayBuffer::new(&gl);

    //buffer the vertex information to the vao
    vao.bind(gl); 
    
    vbo.bind(gl);
    vbo.buffer_data(gl, &vertices, STATIC_DRAW);
    
    // ebo.bind(gl);
    // ebo.buffer_data(gl,&indices, STATIC_DRAW);
    
    Vertex::init_attrib_pointers(&gl);
    
    vbo.unbind(gl);
    // ebo.unbind(gl);
    
    vao.unbind(gl);
    
    (vao,vbo)
  }
}