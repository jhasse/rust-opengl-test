#version 130

uniform vec2 pos;

in vec2 position;

void main() {
	gl_Position = vec4(pos + position, 0.0, 1.0);
}
