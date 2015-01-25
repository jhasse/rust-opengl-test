#version 130

in vec2 position;
in vec2 texcoord;

out vec2 Texcoord;

uniform mat4 projection;
uniform mat4 modelview;

void main() {
    Texcoord = texcoord;
    gl_Position = projection * modelview * vec4(position, 0.0, 1.0);
}
