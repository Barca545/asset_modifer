use crate::{math::{RayCast, Mat4, inverse, Vec4, Transforms}, ecs::world_resources::ScreenDimensions};

#[derive(Debug, Clone, Copy, Default)]
pub struct MouseInput{
  pub ray: Option<MouseRay>
}

#[derive(Debug, Clone, Copy)]
pub struct MouseRay(pub RayCast);

impl MouseRay {
  pub fn new(x:f64, y:f64, screen_dimensions:&ScreenDimensions, transforms:&Transforms) -> Self {
    let inverse_projection:Mat4 = transforms.projection_transform.inverse();
    let inverse_view:Mat4 = inverse(&transforms.view_transform);

    let ndc_x = 2.0 * x as f32 / screen_dimensions.width as f32 - 1.0; //range [-1,1]
    let ndc_y = 1.0 - (2.0 * y as f32) / screen_dimensions.height as f32; //range [-1,1]

    //get the ray's origin in worldspace
    let origin_ndc:Vec4 = Vec4::new(ndc_x, ndc_y, -1.0, 1.0);

    //convert to viewspace
    let mut ray_origin_viewspace_coordinates:Vec4 = inverse_projection * origin_ndc;
    ray_origin_viewspace_coordinates /= ray_origin_viewspace_coordinates.w;

    //convert to worldspace
    let mut ray_origin_worldspace_coordinates:Vec4 = inverse_view * ray_origin_viewspace_coordinates;
    ray_origin_worldspace_coordinates /= ray_origin_worldspace_coordinates.w;

    let end_ndc:Vec4 = Vec4::new(ndc_x, ndc_y, 0.0, 1.0);

    //convert to viewspace
    let mut ray_end_viewspace_coordinates:Vec4 = inverse_projection * end_ndc;
    ray_end_viewspace_coordinates /= ray_end_viewspace_coordinates.w;

    //convert to worldspace
    let mut ray_end_worldspace_coordinates:Vec4 = inverse_view * ray_end_viewspace_coordinates;
    ray_end_worldspace_coordinates /= ray_end_worldspace_coordinates.w;

    MouseRay(RayCast::new(
      ray_origin_worldspace_coordinates.xyz(),
      ray_end_worldspace_coordinates.xyz()
    ))
  }
}