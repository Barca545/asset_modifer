use crate::polygons::Terrain;


#[derive(Debug, Default, Clone)]
pub struct GridInput{
  pub target_cells: Vec<usize>,
  pub setting_terrain:Option<Terrain>
}

