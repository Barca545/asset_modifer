#[derive(Debug, Default)]
pub struct CameraInput{
  pub right:bool,
  pub left:bool,
  pub up:bool,
  pub down:bool,
  // pub zoom:Zoom
  pub zoom:Option<f32>
}