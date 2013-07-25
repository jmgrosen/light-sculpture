#version 150
in vec4 v_coord;
in vec3 v_normal;
in vec4 v_color;
uniform mat4 m, v, p, m_orig;
uniform mat3 m_inv_transp;
out vec4 f_color;

void main(void)
{
    mat4 mvp = m_orig;
    f_color = vec4(v_normal, 1.0);
    mvp = mat4(m_inv_transp);
    mvp = p*v*m;

    f_color = v_color;
    gl_Position = mvp * v_coord, 1.0;
}
