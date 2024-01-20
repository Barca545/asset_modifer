use super::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
  pub position:Vec3,
  pub target:Vec3,
  pub up:Vec3,
  // front:Vec3
}

impl Camera {
  pub fn new() -> Self {
    //up can't be parallel to the view direction?
    // https://community.khronos.org/t/glulookat-and-a-tile-based-top-down-view/64849
    let world_up:Vec3 = Vec3::new(0.0, 0.0, 1.0);

    let x = 0.0;
    let z = 0.0;
    let y = 7.0;

    let front:Vec3 = Vec3::new(-x, -y, -z);

    let position:Vec3 = Vec3::new(x, y, z);

    let right:Vec3 = front.cross(&world_up).normalize();
    let up:Vec3 = right.cross(&front).normalize();
    let target:Vec3 = position + front;

    Camera { 
      position, 
      target, 
      up, 
      // front 
    }
  }
}

impl Default for Camera {
  fn default() -> Self {
    Self::new()
  }
}