#version 330

uniform sampler2D tex;

in vec4 fs_color;
in vec2 fs_uv;

out vec4 frag_color;

void main() {
    frag_color = texture(tex, fs_uv) * fs_color;
}
