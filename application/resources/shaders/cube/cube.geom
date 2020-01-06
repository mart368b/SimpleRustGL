#version 330 core
layout (lines_adjacency) in;
layout (triangle_strip, max_vertices = 6) out;

in VertexOut {
    float Amount;
} vertex_in[]; 

out VertexData {
    vec2 position;
    flat vec4 Amount;
    flat bool right;
} vertex_out;

void main() {
    vertex_out.Amount = vec4(
        vertex_in[0].Amount,
        vertex_in[1].Amount,
        vertex_in[2].Amount,
        vertex_in[3].Amount
    );

    int active = 0;
    for (int i = 0; i < vertex_in.length(); i++) {
        if (vertex_in[i].Amount > 0) {
            active += 1;
        }
    }
    
    if (active > 0) {
        gl_Position = gl_in[0].gl_Position;
        vertex_out.position = vec2(0, 0);
        EmitVertex();
        gl_Position = gl_in[1].gl_Position;
        vertex_out.position = vec2(1, 0);
        EmitVertex();
        gl_Position = gl_in[2].gl_Position;
        vertex_out.position = vec2(1, 1);
        vertex_out.right = true;
        EmitVertex();
        
        gl_Position = gl_in[0].gl_Position;
        vertex_out.position = vec2(0, 0);
        EmitVertex();
        gl_Position = gl_in[2].gl_Position;
        vertex_out.position = vec2(1, 1);
        EmitVertex();
        gl_Position = gl_in[3].gl_Position;
        vertex_out.right = false;
        vertex_out.position = vec2(0, 1);
        EmitVertex();
    }
    

    EndPrimitive();
}