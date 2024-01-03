use std::{path::Path, collections::HashMap};

use eyre::Result;

use crate::render::Vertex;

pub fn load_object_outline(){
  //load the shape in
  //move it up so it is at zero by shifting it up by the lowest y value like in the other program
  //then discard every vert with y>0.0
}

//add the ability to load normally so I can overlay stuff
//copy paste vert loader from engine
pub fn load_object(name:&str) -> Result<(Vec<Vertex>,Vec<u32>)> {
  let path_string = name_to_path_string(name, "obj");
  let path = Path::new(&path_string);
  
  let load_options = &tobj::LoadOptions {
    single_index: true,
    triangulate: true,
    ..Default::default()
  };

  let mut vertices = vec![];
  let mut indices = vec![];
  let mut lowest_y = 0.0;
  let mut unique_vertices = HashMap::new();
  //this eventually is where the materials come from (the second part of the tuple)

  //maybe just make the thing a solid color and don't worry about fighting textures so much rn
  //Fix textures:
  //--load in texture from the mtl file or wherever or figure out how to export a texture as a jpg
  //--fix wrapping
  //----UV wrapping?
  //----first thing to check is if the loaded textures match the textures the gpu is getting 
  
  let (models,_) = tobj::load_obj(path, load_options)?;
  
  for model in &models{
    let mesh = &model.mesh;

    for index in &mesh.indices{
      let position_offset = (index * 3) as usize;
      let texture_offset = (index * 2) as usize;
      
      let position = [
        mesh.positions[position_offset],
        mesh.positions[position_offset + 1],
        mesh.positions[position_offset + 2],
      ];
      if position[1] < lowest_y{
        lowest_y = position[1];
      }

      let texture = [
        mesh.texcoords[texture_offset],
        mesh.texcoords[texture_offset + 1]
      ];

      let vertex = Vertex::new(position, texture);
      
      if let Some(index) = unique_vertices.get(&vertex){
        indices.push(*index as u32)
      }
      else {
        let index = vertices.len();
        unique_vertices.insert(vertex, index);
        vertices.push(vertex);
        indices.push(index as u32);
      }
    }
  }
  
  for vertex in vertices.iter_mut(){
    vertex.pos[1] += lowest_y.abs();
  }

  Ok((vertices,indices))
}

fn name_to_path_string(name:&str, extension:&str) -> String {
  let root_dir = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned();
  let path_string = root_dir + name + "." + extension;
  path_string
}