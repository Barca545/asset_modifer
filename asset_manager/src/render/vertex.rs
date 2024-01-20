use gl::{
  types::{GLint, GLuint, GLvoid},
  Gl, FALSE, FLOAT
};

use std::mem::size_of;
use std::hash::{Hash, Hasher};

//Potential colors for a vertex
//confirm how to make them accessible
const RED:[f32;4] = [1.0,0.0,0.0,1.0];

#[derive(Debug, Clone, Copy)]
#[repr(C)]
// #[repr(C, packed)]
//just make Vertex hold color and all the other data tbh 
//I can just skip it for the systems that don't need it
//can revisit if it becomes a problem but 
//I actually do want color because I want te option to give the shape a solid fill
//actually the above might be a shader thing not a vertex thing
//idk if there is actually a reason to keep color as an option
//could use a vertex builder patter if I really wanted ig
pub struct Vertex {
  pub pos:[f32;3],
  pub txt:[f32;2],
  // pub clr:Option<[f32;3]>
  pub clr:[f32;3],
  pub material_id: usize
}

impl PartialEq for Vertex {
  fn eq(&self, other: &Self) -> bool {
    //Test that the position and txt coords are the same. 
    //No need to check for color because that is not stored in the .obj file.
    self.pos == other.pos
    && self.txt == other.txt
  }
}

impl Eq for Vertex {}

impl From<(f32, f32, f32, f32, f32)> for Vertex {
  fn from(value:(f32, f32, f32, f32, f32)) -> Self {
    let pos:[f32;3] = [value.0, value.1, value.2];
    let txt:[f32;2] = [value.3, value.4];
    // let clr = None;
    let clr = [0.0, 0.0, 0.0];
    let material_id = 0;
    Self::new(pos, txt, clr, material_id)
  }
}

impl From<(f32, f32, f32)> for Vertex {
  fn from(value:(f32, f32, f32)) -> Self {
    let pos:[f32;3] = [value.0, value.1, value.2];
    let txt:[f32;2] = [0.0, 0.0];
    let clr = [0.0, 0.0, 0.0];
    let material_id = 0;
    Self::new(pos, txt, clr, material_id)
  }
}


impl Hash for Vertex{
  fn hash<H:Hasher>(&self, state:&mut H){
    self.pos[0].to_bits().hash(state);
    self.pos[1].to_bits().hash(state);
    self.pos[2].to_bits().hash(state);

    self.txt[0].to_bits().hash(state);
    self.txt[1].to_bits().hash(state);
  }
}

impl Vertex {
  // pub fn new(pos:[f32;3], txt:[f32;2], clr:Option<[f32;3]>) -> Self {
  //   Vertex { pos, txt, clr }
  // }

  pub fn new(pos:[f32;3], txt:[f32;2], clr:[f32;3], material_id: usize) -> Self {
    Vertex { pos, txt, clr, material_id }
  }

  pub fn color(&mut self, color:[f32;3]) -> Self {
    self.clr = color;
    *self
  }

  pub fn init_attrib_pointers(gl:&Gl) {
    let stride = size_of::<Self>();

    //shape
    let position = 0;
    let position_offset = 0;
    Self::define_vertex_attrib_pointer(gl, stride, position, position_offset, 3);

    //texture
    let pos_size = size_of::<[f32;3]>();
    let texture = 1;
    let texture_offset = position_offset + pos_size;
    Self::define_vertex_attrib_pointer(gl, stride, texture, texture_offset, 2);
  
    //color
    let color = 2;
    let txt_size = size_of::<[f32;2]>();
    let color_offset = texture_offset + txt_size;
    Self::define_vertex_attrib_pointer(gl, stride, color, color_offset, 3);
  }

  fn define_vertex_attrib_pointer(gl:&Gl, stride:usize, location:usize, offset:usize, tuple_size:GLint) {
    //why does GITGD (https://github.com/amengede/OpenGL-for-Beginners/blob/main/week%2006%20design%20patterns/4%20entity%20component%20system/src/controller/app.cpp#L12)
    //have EnableVertexAttribArray after VertexAttribPointer?
    unsafe {
      gl.EnableVertexAttribArray(location as GLuint);
      gl.VertexAttribPointer(location as GLuint, tuple_size, FLOAT, FALSE, stride as GLint, offset as *const GLvoid);
    }
  }
}

