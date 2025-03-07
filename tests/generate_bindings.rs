//! Tests for generating bindings
//!

use spirv_bindgen::Spirv;

#[test]
fn generate_bindings_spec_constants() {
    let spirv = Spirv::try_from_bytes(
        "spec_constants",
        include_bytes!("./data/spv/spec_constants.spv").as_slice(),
    );
    println!("{}", spirv.pretty_string());
}

#[test]
fn generate_bindings_render_capture() {
    let spirv = Spirv::try_from_bytes(
        "render_capture",
        include_bytes!("./data/spv/render_capture.spv").as_slice(),
    );
    println!("{}", spirv.pretty_string());
}

#[test]
fn generate_bindings_render_line() {
    let spirv = Spirv::try_from_bytes(
        "render_line",
        include_bytes!("./data/spv/render_line.spv").as_slice(),
    );
    println!("{}", spirv.pretty_string());
}

#[test]
fn generate_bindings_maximum_reduction() {
    let spirv = Spirv::try_from_bytes(
        "maximum_reduction",
        include_bytes!("./data/spv/maximum_reduction.spv").as_slice(),
    );
    println!("{}", spirv.pretty_string());
}
