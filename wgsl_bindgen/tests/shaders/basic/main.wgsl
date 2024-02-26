#import bindings;
#import types::{Fp64};

struct Style {
    color: vec4f,
    width: f32,
}

@group(0) @binding(0)
var<storage, read_write> buffer: array<f32>;

var<push_constant> const_style: Style;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    buffer[id.x] *= 2 * bindings::ONE * const_style.color.a * const_style.width;
}
