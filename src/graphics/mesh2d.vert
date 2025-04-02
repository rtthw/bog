in vec2 position;
in vec4 color;
uniform mat4 model;
uniform mat4 view_projection;
out vec4 v_color;

void main() {
    gl_Position = view_projection * model * vec4(position, 0.0, 1.0);

    v_color = color;
}
