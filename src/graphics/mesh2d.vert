in vec2 a_pos;
in vec4 a_color;

out vec4 v_color;

void main() {
    gl_Position = vec4(a_pos.x, a_pos.y, 0.0, 1.0);
    v_color = a_color;
}
