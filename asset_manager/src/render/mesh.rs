use gl::{Gl, STATIC_DRAW};

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