use std::{path::{Path, PathBuf}, collections::HashMap, fs::File, io::Read, ffi::CString};

use eyre::Result;
use image::{io::Reader, DynamicImage};

use crate::render::Vertex;
use crate::errors::FilesystemErrors;

// also look into exporting a 2d outline from Blender
// https://blender.stackexchange.com/questions/60600/project-3d-object-on-a-plane
//arguably not a loader function so consider moving
// (vertices, indices):(Vec<Vertex>,Vec<u32>)
pub fn load_object_outline(name:&str) -> Result<(Vec<Vertex>,Vec<u32>)> {
  // //Create new vectors to hold the outline verts and indices
  // let mut outline_vertices = Vec::default();
  // let mut outline_indices = Vec::default();

  // //Sort the vertices
  // for index in indices{
  //   let index = index as usize;
  //   let mut vertex = vertices[index];
  //   let vertex_y = vertex.pos[1];
    
  //   let outline_index = index as u32;
  //   vertex.pos[1] = 0.0;
  //   outline_vertices.push(vertex);
  //   outline_indices.push(outline_index);


  //   //Discard every vertex with not on the XZ-plane and every vertex inside the shape

  //   //unsure the second part is necessary, if it is, I need a second test
  //   // if vertex_y > 0.0 {
  //   //   let outline_index = index as u32;
  //   //   outline_vertices.push(vertex);
  //   //   outline_indices.push(outline_index);
  //   // }  
  // }
  // Ok((outline_vertices,outline_indices))
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
  
  let (models,_) = tobj::load_obj(path, load_options)?;
  
  for model in &models{
    let mesh = &model.mesh;

    for index in &mesh.indices{
      let position_offset = (index * 3) as usize;
      let texture_offset = (index * 2) as usize;
      
      let position = [
        mesh.positions[position_offset],
        // mesh.positions[position_offset + 1],
        0.0,
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

pub fn load_image(name:&str, extension:&str) -> Result<DynamicImage> {
  let path = name_to_pathbuff(name, extension);
  //I don't want to panic I want to print it to the UI
  let image = Reader::open(path)
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToLoadImage }))
    .decode()
    .unwrap_or_else(|_| panic!("{}", { FilesystemErrors::FailedToDecodeImage }));
  Ok(image)
}

pub fn load_cstring(name:&str, extension:&str) -> Result<CString> {
  let mut file = File::open(name_to_pathbuff(name, extension))?;

  let mut buffer:Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);

  file.read_to_end(&mut buffer)?;

  if buffer.iter().find(|i| **i == 0).is_some() {
    return Err(FilesystemErrors::FileContainsNil.into());
  }
  Ok(unsafe { CString::from_vec_unchecked(buffer) })
}

pub fn load_shader(name:&str, extension:&str) -> Result<CString> {
  let shader = load_cstring(name, extension)?;
  Ok(shader)
}

fn name_to_path_string(name:&str, extension:&str) -> String {
  let root_dir = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned();
  let path_string = root_dir + name + "." + extension;
  path_string
}

fn name_to_pathbuff(name:&str, extension:&str) -> PathBuf {
  let root_dir = "C:/Users/Jamari/Documents/Hobbies/Coding/deux/target/debug/assets/".to_owned();
  let path:PathBuf = PathBuf::from(root_dir + name + "." + extension);
  path
}