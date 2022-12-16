#version 300 es

in vec2 position;

mat2 iso_proj = mat2(
    1.0, -0.5,
    1.0, 0.5
);

void main() {
    gl_Position = vec4(iso_proj * position, 0.0, 1.0);
}
