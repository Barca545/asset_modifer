use super::{camera_input::CameraInput, MouseInput, grid_input::GridInput, mode::Mode};

#[derive(Debug, Default)]
pub struct InputManager{
  pub mode: Mode,
  pub mouse: MouseInput,
  pub camera: CameraInput,
  pub grid: GridInput
}

impl InputManager {
  pub fn new() -> Self {
    InputManager{
      mode: Mode::Selecting,
      mouse: MouseInput::default(),
      camera: CameraInput::default(),
      grid: GridInput::default()
    }
  }
}