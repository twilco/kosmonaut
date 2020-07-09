#version 330 core
layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>
out vec2 TexCoords;

uniform mat4 projection;

void main()
{
    // TODO: Use orthographic projections instead of NDC coordinates calculated on the CPU.
    // gl_Position = projection * vec4(vertex.xy, 0.0, 1.0);
    gl_Position = vec4(vertex.xy, 0.0, 1.0);
    TexCoords = vertex.zw;
}