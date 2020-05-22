#version 330 core
    
layout (location = 0) in vec2 Position;
layout (location = 1) in float Amount;

out VertexOut {
    float Amount;
} vertex_out;

void main()
{
    gl_Position = vec4(Position.xy, 1.0, 1.0);
    vertex_out.Amount = Amount;
}