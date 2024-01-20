use super::{camera_input::CameraInput, MouseInput, grid_input::GridInput};

#[derive(Debug, Default)]
pub struct InputManager{
  pub mouse: MouseInput,
  pub camera: CameraInput,
  pub grid: GridInput
}

impl InputManager {
  pub fn new() -> Self {
    InputManager{
      mouse: MouseInput::default(),
      camera: CameraInput::default(),
      grid: GridInput::default()
    }
  }
}