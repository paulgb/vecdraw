#version 450

layout(location=0) in vec2 a_start;
layout(location=1) in vec2 a_end;
layout(location=2) in vec4 a_color;
layout(location=3) in float a_width;

layout(location=0) out vec4 v_color;
layout(location=1) out vec2 v_edge;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 u_transform;
};

void main() {
    vec2 line = normalize(a_end - a_start);
    vec2 perp = vec2(line.y, -line.x);

    vec2 c1 = a_start - perp * a_width;
    vec2 c2 = a_start + perp * a_width;
    vec2 c3 = a_end - perp * a_width;
    vec2 c4 = a_end + perp * a_width;

    switch (gl_VertexIndex) {
        case 0:
        gl_Position = vec4(c1, 0., 1.);
        v_edge = vec2(0., 0.);
        break;
        case 1:
        case 3:
        gl_Position = vec4(c2, 0., 1.);
        v_edge = vec2(0., 1.);
        break;
        case 2:
        case 4:
        gl_Position = vec4(c3, 0., 1.);
        v_edge = vec2(1., 0.);
        break;
        case 5:
        gl_Position = vec4(c4, 0., 1.);
        v_edge = vec2(0., 0.);
    }

    gl_Position = u_transform * gl_Position;

    v_color = a_color;
}
