#version 450

layout(location=0) in float a_location;
layout(location=1) in vec4 a_color;
layout(location=2) in float a_width;
layout(location=3) in uint a_vertical;

layout(location=0) out vec4 v_color;
layout(location=1) out vec2 v_edge;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 u_transform;
};

void main() {
    vec2 c1, c2, c3, c4;

    vec4 scaled = u_transform * vec4(a_location, a_location, 0.0, 1.0);

    if (a_vertical == 0) {
        c1 = vec2(scaled.x - a_width, -1.0);
        c2 = vec2(scaled.x + a_width, -1.0);
        c3 = vec2(scaled.x - a_width, 1.0);
        c4 = vec2(scaled.x + a_width, 1.0);
    } else {
        c1 = vec2(-1.0, scaled.y - a_width);
        c2 = vec2(-1.0, scaled.y + a_width);
        c3 = vec2(1.0, scaled.y - a_width);
        c4 = vec2(1.0, scaled.y + a_width);
    }

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

    v_color = a_color;
}
