use gl::{Gl, TRIANGLES, UNSIGNED_INT, types::{GLsizei, GLvoid, GLint}, LINES, POINTS, LINE_STRIP, LINE_LOOP};

// use crate::polygons::GridMesh;

use super::{Mesh, mesh::GridMesh};

//make a file just holding the draw mode functions

//test by just doing the current render then start making it draw_elements
pub fn render_mesh(gl:&Gl, mesh:&Mesh){
  let texture = &mesh.texture;
  let vao = &mesh.vao;
  let indices = &mesh.indices;
  
  //do I bind texture before or after vao
  texture.bind(gl);
  vao.bind(gl);


  //Bind the model transform
  unsafe {
    gl.DrawElements(
      TRIANGLES,
      indices.len() as GLsizei,
      UNSIGNED_INT,
      indices.as_ptr() as *const GLvoid
    );
  }

  vao.unbind(gl);
} 

pub fn render_lines(gl:&Gl, grid_mesh:&GridMesh){
  let vertices = &grid_mesh.lines.vertices;
  let vao = &grid_mesh.lines.vao;

  vao.bind(gl);

  unsafe {
    gl.DrawArrays(
      LINES,
      0,
      vertices.len() as GLint
    );
  }
}

pub fn render_cells(gl:&Gl, grid_mesh:&GridMesh){
  let vertices = &grid_mesh.cells.vertices;
  let vao = &grid_mesh.cells.vao;

  vao.bind(gl);

  unsafe {
    gl.DrawArrays(
      TRIANGLES,
      0,
      vertices.len() as GLint
    );
  }
}