#version 150

uniform vec3 triangleColor;

in vec2 texcoord;

out vec4 outColor;

uniform sampler2D tex;

void main() {
    outColor = texture(tex, texcoord);
}
