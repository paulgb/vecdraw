struct VertexOutput {
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] coord: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[block]]
struct Uniforms {
    transform: mat4x4<f32>;
};

[[group(0), binding(0)]]
var uniforms: Uniforms;

let corners: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1., -1.),
    vec2<f32>(1., -1.),
    vec2<f32>(-1., 1.),
    vec2<f32>(1., -1.),
    vec2<f32>(-1., 1.),
    vec2<f32>(1., 1.),
);

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32,
    [[location(0)]] position: vec4<f32>,
    [[location(1)]] color: vec4<f32>,
    [[location(2)]] radius: f32,
) -> VertexOutput {
    var out: VertexOutput;

    out.color = color;
    out.coord = corners[in_vertex_index];
    out.position = uniforms.transform * vec4<f32>(
        position.x + radius * out.coord.x,
        position.y + radius * out.coord.y,
        0.,
        1.
    );

    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let r: f32 = dot(in.coord, in.coord);
    let delta: f32 = fwidth(r);

    let alpha: f32 = 1.0 - smoothStep(1.0 - delta*2., 1.0, r);

    if (alpha < 0.01) {
        discard;
    }

    return in.color * alpha;
}