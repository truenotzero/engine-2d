use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::Api;
use gl_generator::Fallbacks;
use gl_generator::GlobalGenerator;
use gl_generator::Profile;
use gl_generator::Registry;

extern crate gl_generator;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}
