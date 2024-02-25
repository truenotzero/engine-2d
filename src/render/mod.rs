// before using any render functionality this module must be initialized
// call `render::init`
// each API has a specific requirement, so check each api's `init`  documentation

pub(self) mod api;
pub use api::init;
pub use api::clear;
pub use api::window;

pub mod shader;
pub mod sprite;
pub mod texture;

// w is the lifetime of the canvas in which
// the context will be created
#[derive(Clone, Copy)]
pub struct Context<'w>(std::marker::PhantomData<&'w ()>);
