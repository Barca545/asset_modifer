use super::{radians, aliases::{Perspective, identity, translate, scale, look_at}, camera::Camera, Mat4, Vec3};

//might make the most sense to make the model transform something the entity holds instead of something the global struct holds
//also restructure this and the camera class to match the ecs one in GITGD's tutorial repo

#[derive(Debug, Clone, Copy)]
pub struct Transforms {
  pub projection_transform:Perspective,
  pub view_transform:Mat4,
  //fov and camera will be used when I make a camera system
  // fov:f32,
  pub camera:Camera
}

impl Transforms {
  pub fn new(aspect:&f32) -> Self {
    let fov = radians(45.0);
    let camera = Camera::new();
    let view_transform:Mat4 = Self::calculate_view_transform(&camera);
    let projection_transform = Self::calculate_projection_transform(fov, aspect);

    Transforms {
      projection_transform,
      view_transform,
      // fov,
      camera
    }
  }

  pub fn update_view(&mut self, camera:&Camera){
    self.view_transform = Self::calculate_view_transform(camera);
    self.camera = *camera;
  }

  fn calculate_view_transform(camera:&Camera) -> Mat4 {
    let view:Mat4 = look_at(camera);
    view
  }

  fn calculate_projection_transform(fov:f32, aspect:&f32) -> Perspective {
    //Do this without cloning?
    let projection = Perspective::new(aspect.clone(), fov, 0.1, 100.0);
    projection
  }
}

pub fn calculate_model_transform(position:&Vec3, scale_factor:f32) -> Mat4 {
  let model:Mat4 = identity();
  let model:Mat4 = translate(&model, position);
  let model:Mat4 = scale(&model, scale_factor);
  model
}