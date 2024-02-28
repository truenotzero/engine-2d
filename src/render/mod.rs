// before using any render functionality this module must be initialized
// call `render::init`
// each API has a specific requirement, so check each api's `init`  documentation

mod api;
pub use api::clear;
pub use api::init;

pub mod shader;
pub mod sprite;
pub mod texture;
pub mod window;
