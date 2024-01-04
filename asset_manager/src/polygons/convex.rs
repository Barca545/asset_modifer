use crate::{math::{Point, Vec3}, render::Vertex};

pub struct ConvexPolygon{
  object_position:Point,
  //really I don't want Vertex, I want points this is not for rendering
  //then in my engine I can impl From Point for Vec3 or Into Vec3 for point or  even just a to function all else fails
  vertices_positions:Vec<Point>
}

impl ConvexPolygon {
  pub fn new(object_position:Point, vertices:Vec<Vertex>) -> Self {
    let mut vertices_positions = Vec::default();

    for vertex in vertices{
      let vertex = Point::from(vertex);
      vertices_positions.push(vertex)
    }

    Self{
      object_position,
      vertices_positions
    }
  }
}

//I also need a struct version of this for rendering

/// Algorithm for finding the convex hull around a set of points. 
/// Adapted from: https://web.archive.org/web/20180409175413/http://www.ahristov.com/tutorial/geometry-games/convex-hull.html
pub fn quick_hull(points:Vec<Point>){
  
}

///Returns the relative distance between Point` P and the segment between `Point` A and `Point` B.
fn pseudo_dist(a:Point, b:Point, p:Point) -> f32{
  //what is the math happening here
  let abx = b.x - a.x;
  let abz = b.z - b.z;

  let distance = abx * (a.z - p.z) - abz * (a.x - p.x);
  
  distance.abs()
}


// you can tell the orientation of 3 points in space using the cross product because sin is an odd  function 


// so like in the picture if you take the cross product of those points you can tell if going from B to C is moving counterclockwise (the angle is positive resulting in a positive cross product) or counterclockwise (negative cross product and angle)

// because sin(-x) = -sin(x)
// https://math.stackexchange.com/a/285356

///Calculates the orientation of `Point` P with respect to the segment between `Point` A and `Point` B.
/// Returns result > 0 if P is counterclockwise from AB, result < 0 if P is to clockwise from AB, and 0 if the points are colinear.
/// Further reading: https://math.stackexchange.com/a/285356
fn orientation(a:Point, b:Point, p:Point) -> f32 {
  //ABxAP
  //I think this should use z not y
  let cross = (b.x - a.x) * (p.z - a.z) - (b.z - a.z) * (p.x - a.x);
  cross
}

///Based on: https://www.researchgate.net/publication/220868874_Concave_hull_A_k-nearest_neighbours_approach_for_the_computation_of_the_region_occupied_by_a_set_of_points.
/// Prefer number of neighbors = k = 3.
fn concave_hull(points:Vec<Vec3>, k:u32) -> Vec<Point>{
  //set the neighbors as a mutable variable 
  let mut k = k;
  
  //find the minimum x
  let min_y_index = min_y(&points);

  //Copy the points so it can be mutated
  let mut temp_points = points.clone();

  //Initalize the hull with minimum x point as the first point and exclude the first point from the points vec
  let first_point = temp_points.remove(min_y_index);
  let hull = vec![first_point];

  //Initalize the step and angle 
  //why^
  let step = 2;
  let mut previous_angle = 0.0;
  let mut current_point = first_point;

  //Find the nearest neighboring point (could optimize here)
  while current_point != first_point || step == 2 && temp_points.len()>0{
    if step == 5{
      //add the first point back to the dataset
    }
    let mut nearest_points = nearest_points(current_point, &temp_points, k );
    let sorted_points = sort_by_angle(current_point, &mut nearest_points, &mut previous_angle);
    
    //Select the first neighboring point that does not intersect any of the polygon edges
    //what is "its"
    let its = true;
    let mut index = 0;
    while its == true && index < sorted_points.len(){
      index += 1;
    }
    //I stopped on line 20 of this:file:///C:/Users/Jamari/Downloads/ConcaveHull_ACM_MYS.pdf
  }

  
  //If the resulting hull is invalid because ____ increase k and try again
  todo!()
}

///Iterates over a set of points and returns the index of the point with the smallest x;
//the example algorithm wants min y tho?
fn min_x(points:&Vec<Vec3>) -> usize {
  let mut min_x = 0.0;
  let mut min_y = 0.0;
  let mut min_index = 0;
  
  for (index, point) in points.iter().enumerate(){
    if point.x < min_x{
      min_x = point.x;
      min_index = index;
    }
    //If the points have the same x value take the one with the smallest y as the new minimum
    else if point.x == min_x{
      if point.y < min_y{
        min_x = point.x;
        min_y = point.y;
        min_index = index;
      }
    }
  }
  min_index
}

fn min_y(points:&Vec<Vec3>) -> usize {
  let mut min_y = 0.0;
  let mut min_x = 0.0;
  let mut min_index = 0;
  
  for (index, point) in points.iter().enumerate(){
    if point.y < min_y{
      min_y = point.y;
      min_index = index;
    }
    //If the points have the same x value take the one with the smallest y as the new minimum
    else if point.y == min_y{
      if point.x < min_x{
        min_y = point.y;
        min_x = point.x;
        min_index = index;
      }
    }
  }
  min_index
}

///Finds the k nearest neighbors in the set to the `current_point`;
fn nearest_points(current_point:Vec3, points:&Vec<Vec3>, k:u32) -> Vec<Vec3> {
  todo!()
}

///Sorts the given vector of points in descending order of the size of the angle between them and the current point.
fn sort_by_angle(current_point:Vec3, points:&mut Vec<Vec3>, previous_angle:&mut f32) -> Vec<Vec3>{
  todo!()
}