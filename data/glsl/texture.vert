#version 150

in vec2 position;
in vec2 texcoord;

out vec2 outTexcoord;

void main() {
    outTexcoord = texcoord;
	gl_Position = vec4(position, 0.0, 1.0);
}
