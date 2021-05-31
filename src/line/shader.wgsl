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
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
    [[location(0)]] start: vec2<f32>,
    [[location(1)]] end: vec2<f32>,
    [[location(2)]] color: vec4<f32>,
    [[location(3)]] width: f32,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = color;

    let line: vec2<f32> = normalize(end - start);
    let perp: vec2<f32> = vec2<f32>(line.y, -line.x);

    switch (i32(in_vertex_index)) {
        case 0: {
            let c: vec2<f32> = start - perp * width;
            out.position = vec4<f32>(c, 0., 1.);
            out.edge = vec2<f32>(0., 0.);
        }
        case 1: {
            fallthrough;
        }
        case 3: {
            let c: vec2<f32> = start + perp * width;
            out.position = vec4<f32>(c, 0., 1.);
            out.edge = vec2<f32>(0., 1.);
        }
        case 2: {
            fallthrough;
        }
        case 4: {
            let c: vec2<f32> = end - perp * width;
            out.position = vec4<f32>(c, 0., 1.);
            out.edge = vec2<f32>(1., 0.);
        }
        case 5: {
            let c: vec2<f32> = end + perp * width;
            out.position = vec4<f32>(c, 0., 1.);
            out.edge = vec2<f32>(0., 0.);
        }
    }

    out.position = uniforms.transform * out.position;

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