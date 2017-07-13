#version 140

uniform mat4 flat_projection;
uniform mat4 ortho_projection;
uniform mat4 matrix;

in vec2 position;

void main() {
    gl_Position = flat_projection * vec4(position, 0.0, 1.0);
}
