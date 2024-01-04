use gl;
use gl::{
  types::{GLsizeiptr, GLuint, GLvoid, GLenum},
  Gl, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER
};
use std::{marker::PhantomData, mem::size_of};

pub type ArrayBuffer = Buffer<Array>;
pub type ElementArrayBuffer = Buffer<ElementArray>;

pub trait BufferType {
  const BUFFER_TYPE:GLuint;
}

#[derive(Debug, Clone, Copy)]
pub struct Array;
impl BufferType for Array {
  const BUFFER_TYPE:GLuint = ARRAY_BUFFER;
}
#[derive(Debug, Clone, Copy)]
pub struct ElementArray;
impl BufferType for ElementArray {
  const BUFFER_TYPE:GLuint = ELEMENT_ARRAY_BUFFER;
}

#[derive(Debug, Clone, Copy)]
pub struct Buffer<B> {
  pub buffer_obj:GLuint,
  _marker:PhantomData<B>
}

impl<B> Buffer<B>
where B: BufferType
{
  pub fn new(gl:&Gl) -> Buffer<B> {
    let mut buffer_obj:GLuint = 0;
    unsafe { gl.GenBuffers(1, &mut buffer_obj) }

    Buffer {
      buffer_obj, //can be a vbo or ebo
      _marker:PhantomData
    }
  }

  pub fn bind(&self,gl:&Gl) {
    unsafe {gl.BindBuffer(B::BUFFER_TYPE, self.buffer_obj)}
  }

  pub fn unbind(&self,gl:&Gl) {
    unsafe {gl.BindBuffer(B::BUFFER_TYPE, 0)}
  }

  pub fn buffer_data<T>(&self, gl:&Gl, data:&[T], usage:GLenum) {
    unsafe {
      gl.BufferData(
        B::BUFFER_TYPE,
        (data.len() * size_of::<T>()) as GLsizeiptr,
        data.as_ptr() as *const GLvoid,
        usage
      )
    }
  }
}

#[derive(Debug, Clone)]
pub struct VertexArray {
  vao:GLuint
}

impl VertexArray {
  pub fn new(gl:&Gl) -> VertexArray {
    let mut vao:GLuint = 0;
    unsafe {gl.GenVertexArrays(1, &mut vao)}
    VertexArray {
      vao 
    }
  }

  pub fn bind(&self, gl:&Gl) {
    unsafe {gl.BindVertexArray(self.vao)}
  }

  pub fn unbind(&self, gl:&Gl) {
    unsafe {gl.BindVertexArray(0)}
  }
}