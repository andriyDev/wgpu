@group(0)
@binding(0)
var<storage, read_write> v_indices: array<u32>; // this is used as both input and output for convenience

struct Number1 {
    value: u32,
}
struct Number2 {
    padding: u32,
    value: u32,
}

var<push_constant> multiplier: Number1;
var<push_constant> bias: Number2;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices[global_id.x] = v_indices[global_id.x] * multiplier.value + bias.value;
}
