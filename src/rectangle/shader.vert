#version 450

layout(location=0) in vec2 a_upper_left;
layout(location=1) in vec2 a_lower_right;
layout(location=2) in vec4 a_color;

layout(location=0) out vec4 v_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 u_transform;
};

void main() {
    switch (gl_VertexIndex) {
        case 0:
            gl_Position = vec4(a_upper_left, 0., 1.);
            break;
        case 1:
        case 3:
            gl_Position = vec4(a_upper_left.x, a_lower_right.y, 0., 1.);
            break;
        case 2:
        case 4:
            gl_Position = vec4(a_lower_right.x, a_upper_left.y, 0., 1.);
            break;
        case 5:
            gl_Position = vec4(a_lower_right, 0., 1.);
    }

    gl_Position = u_transform * gl_Position;

    v_color = a_color;
}
