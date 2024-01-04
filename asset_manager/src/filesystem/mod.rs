mod loader;
mod parser;

pub use self::{
    loader::{load_image,load_shader, load_object, load_object_outline},
    parser::create_whitespace_cstring
};