use create::create_gl;
use gl::COLOR_BUFFER_BIT;
use glfw::{Key, Action, Context};

use crate::create::create_window;

mod create;
mod loader;
mod render;

extern crate gl;

fn main() {
  //init glfw
  let (mut glfw, mut window, events) = create_window();
  let gl = create_gl(&mut window);

  unsafe {
    gl.ClearColor(0.3, 0.3, 0.5, 1.0);
  } 

  while !window.should_close(){
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
      }
    }
    unsafe{
      gl.Clear(COLOR_BUFFER_BIT)
    }
    window.swap_buffers();
  }
  
  
  //get object loading 
  //set up rendering
  //--for rendering each vert on the outline should get a little circle showing where it is
  //--circles should be movable, selectable, and deletable
  //--moving them should update the location of the vert
  //set up overlay capability
  //add GUI
  //add file system access
  //set up hot reloading
  //add input handling
  //convert mouse pictures into worldspace
  //convert mouse input to world input
  //add scripts to calculate the relevant hitboxes
  //add saving
  //ensure interoperability with the game
}
