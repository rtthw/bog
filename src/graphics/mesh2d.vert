
uniform vec2 u_screen_size;
in vec2 a_pos;
in vec4 a_color;

out vec4 v_color;

void main() {
    gl_Position = vec4(
                      2.0 * a_pos.x / u_screen_size.x - 1.0,
                      1.0 - 2.0 * a_pos.y / u_screen_size.y,
                      0.0,
                      1.0);
    v_color = a_color;
}
