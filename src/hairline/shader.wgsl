struct VertexOutput {
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] edge: vec2<f32>;
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
    [[location(0)]] position: f32,
    [[location(1)]] color: vec4<f32>,
    [[location(2)]] width: f32,
    [[location(3)]] vertical: u32,
) -> VertexOutput {
    var out: VertexOutput;

    var c1: vec2<f32>;
    var c2: vec2<f32>;
    var c3: vec2<f32>;
    var c4: vec2<f32>;

    let scaled: vec4<f32> = uniforms.transform * vec4<f32>(position, position, 0.0, 1.0);

    if (vertical == 0u) {
        c1 = vec2<f32>(scaled.x - width, -1.0);
        c2 = vec2<f32>(scaled.x + width, -1.0);
        c3 = vec2<f32>(scaled.x - width, 1.0);
        c4 = vec2<f32>(scaled.x + width, 1.0);
    } else {
        c1 = vec2<f32>(-1.0, scaled.y - width);
        c2 = vec2<f32>(-1.0, scaled.y + width);
        c3 = vec2<f32>(1.0, scaled.y - width);
        c4 = vec2<f32>(1.0, scaled.y + width);
    }

    switch (i32(in_vertex_index)) {
        case 0: {
            out.position = vec4<f32>(c1, 0., 1.);
            out.edge = vec2<f32>(0., 0.);
        }
        case 1: {
            fallthrough;
        }
        case 3: {
            out.position = vec4<f32>(c2, 0., 1.);
            out.edge = vec2<f32>(0., 1.);
        }
        case 2: {
            fallthrough;
        }
        case 4: {
            out.position = vec4<f32>(c3, 0., 1.);
            out.edge = vec2<f32>(1., 0.);
        }
        case 5: {
            out.position = vec4<f32>(c4, 0., 1.);
            out.edge = vec2<f32>(0., 0.);
        }
    }

    out.color = color;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let dx: f32 = fwidth(in.edge.x);
    let dy: f32 = fwidth(in.edge.y);

    let xcov: f32 = min(clamp(0., 1., in.edge.x / dx), clamp(0., 1., (1. - in.edge.x) / dx));
    let ycov: f32 = min(clamp(0., 1., in.edge.y / dy), clamp(0., 1., (1. - in.edge.y) / dy));
    let alpha: f32 = xcov * ycov;

    return in.color * alpha;
}
