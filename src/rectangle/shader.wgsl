struct VertexOutput {
    [[location(0)]] color: vec4<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[block]]
struct Uniforms {
    transform: mat4x4<f32>;
};

[[group(0), binding(0)]]
var uniforms: Uniforms;

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32,
    [[location(0)]] upper_left: vec2<f32>,
    [[location(1)]] lower_right: vec2<f32>,
    [[location(2)]] color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    switch (i32(in_vertex_index)) {
        case 0: {
            out.position = vec4<f32>(upper_left, 0., 1.); 
        }
        case 1: {
            fallthrough;
        }
        case 3: {
            out.position = vec4<f32>(upper_left.x, lower_right.y, 0., 1.);
        }
        case 2: {
            fallthrough;
        }
        case 4: {
            out.position = vec4<f32>(lower_right.x, upper_left.y, 0., 1.);
        }
        case 5: {
            out.position = vec4<f32>(lower_right, 0., 1.);
        }
    }

    out.position = uniforms.transform * out.position;
    out.color = color;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
