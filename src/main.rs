use glfw::{Key, Action};

use crate::create::create_window;

mod create;
mod loader;

fn main() {
  //init glfw
  let (mut glfw, mut window, events) = create_window();
  while !window.should_close(){
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
      }
    }
  }
  //get gl connected
  //set up relevant structs
  //get object loading 
  //set up rendering
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
