use gl::{Gl, TRIANGLES, UNSIGNED_INT, types::{GLsizei, GLvoid, GLint}};

use super::Mesh;

//test by just doing the current render then start making it draw_elements
pub fn render_mesh(gl:&Gl, mesh:&Mesh){
  let vertices = &mesh.vertices;
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