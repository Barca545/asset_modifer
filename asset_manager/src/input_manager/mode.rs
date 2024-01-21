// stick mode on the input manager
#[derive(Debug, Default, PartialEq)]
pub enum DrawMode{
  #[default]
  Clicking,
  Dragging
}


#[derive(Debug, Default, PartialEq)]
pub enum Mode{
  #[default]
  Selecting,
  Drawing(DrawMode)
}