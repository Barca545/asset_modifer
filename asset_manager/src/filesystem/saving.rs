use std::{fs::File, io::Write};

use crate::polygons::Grid;
use eyre::Result;

//need to figure out how to generate/get the path
//I think I will create resources that hold the path to different stuff

pub fn save_file_dialog(){}

pub fn save_grid(grid:&Grid, path:&str) -> Result<()>{
  let serialized = serde_json::to_string(grid)?;

  let mut file = File::create(path)?;
  file.write_all(serialized.as_bytes())?;

  Ok(())
}