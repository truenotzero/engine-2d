
pub mod shader;
pub mod api;

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}