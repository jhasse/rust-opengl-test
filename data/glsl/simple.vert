#version 130

in vec2 position;

uniform mat4 projection;
uniform mat4 modelview;

void main() {
    gl_Position = projection * modelview * vec4(position, 0.0, 1.0);
}
