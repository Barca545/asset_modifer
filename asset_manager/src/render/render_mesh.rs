use gl::{Gl, TRIANGLES, UNSIGNED_INT, types::{GLsizei, GLvoid, GLint}, LINES};

// use crate::polygons::GridMesh;

use super::{Mesh, mesh::GridMesh};

//test by just doing the current render then start making it draw_elements
pub fn render_mesh(gl:&Gl, mesh:&Mesh){
  let texture = &mesh.texture;
  let vao = &mesh.vao;
  let indices = &mesh.indices;
  
  //do I bind texture before or after vao
  texture.bind(gl);
  vao.bind(gl);
  
  //make a file just holding the draw mode functions

  // //bind the model transform
  // program.set_uniform_matrix4fv(gl, uniform_locations.model, &transforms.get_model_transform(&render_position, 1.0));
  unsafe {
    gl.DrawElements(
      TRIANGLES,
      indices.len() as GLsizei,
      UNSIGNED_INT,
      indices.as_ptr() as *const GLvoid
    );
  }

//   unsafe {
//     gl.DrawArrays(
//       TRIANGLES,
//       0,
//       vertices.len() as GLint,
//     );
//   }
  vao.unbind(gl);
} 

pub fn render_grid_mesh(gl:&Gl, grid_mesh:&GridMesh){
  // let texture = &mesh.texture;
  
  let vertices = &grid_mesh.vertices;
  let vao = &grid_mesh.vao;

  vao.bind(gl);

  unsafe {
    gl.DrawArrays(
      LINES,
      0,
      vertices.len() as GLint
    );
  }
}