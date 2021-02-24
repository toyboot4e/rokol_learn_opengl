#version 330

layout(location=0) in vec3 vs_pos;
layout(location=1) in vec4 vs_color;
layout(location=2) in vec2 vs_uv;

out vec4 fs_color;
out vec2 fs_uv;

void main() {
    gl_Position = vec4(vs_pos, 1.0);
    fs_color = vs_color;
    fs_uv = vs_uv;
}
