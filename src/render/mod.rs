// before using any render functionality this module must be initialized
// call `render::init`
// each API has a specific requirement, so check each api's `init`  documentation

pub(self) mod api;
pub use api::init;

pub mod shader;
pub mod texture;
