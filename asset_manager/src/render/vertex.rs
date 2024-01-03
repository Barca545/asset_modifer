use gl::{
  types::{GLint, GLuint, GLvoid},
  Gl, FALSE, FLOAT
};

use std::mem::size_of;
use std::hash::{Hash, Hasher};


#[derive(Copy, Clone, Debug)]
#[repr(C)]
// #[repr(C, packed)]
//just make Vertex hold color and all the other data tbh 
//I can just skip it for the systems that don't need it
//can revisit if it becomes a problem but 
//I actually do want color because I want te option to give the shape a solid fill
//actually the above might be a shader thing not a vertex thing
pub struct Vertex {
  pub(crate) pos:[f32;3],
  pub(crate) txt:[f32;2]
}

impl PartialEq for Vertex {
  fn eq(&self, other: &Self) -> bool {
    self.pos == other.pos
    && self.txt == other.txt
  }
}

impl Eq for Vertex {}

//could do multiple from impls and make txt and option
impl From<(f32, f32, f32, f32, f32)> for Vertex {
  fn from(value:(f32, f32, f32, f32, f32)) -> Self {
    let pos:[f32;3] = [value.0, value.1, value.2];
    let txt:[f32;2] = [value.3, value.4];
    Self::new(pos, txt)
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
  pub fn new(pos:[f32;3], txt:[f32;2]) -> Self {
    Vertex { pos, txt }
  }

  pub fn init_attrib_pointers(gl:&Gl) {
    let stride = size_of::<Self>();

    //shape
    let position = 0;
    let position_offset = 0;
    Self::define_vertex_attrib_pointer(gl, stride, position, position_offset, 3);

    //texture
    let size = size_of::<[f32;3]>();
    let texture = 1;
    let texture_offset = position_offset + size;
    Self::define_vertex_attrib_pointer(gl, stride, texture, texture_offset, 2);
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
