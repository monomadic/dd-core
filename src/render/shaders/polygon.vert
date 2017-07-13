#version 140

uniform mat4 projection;
uniform mat4 matrix;

in vec2 position;

void main() {
    gl_Position = projection * vec4(position, 0.0, 1.0);
}
