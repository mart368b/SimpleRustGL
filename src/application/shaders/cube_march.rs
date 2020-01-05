use anyhow::Result;

pub use crate::gfx::graphics::*;

pub fn create_cube_vertex_shader() -> Result<VertexShader> {
    Ok(VertexShader::from_source("\
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
    }\
    ")?)
}

pub fn create_cube_geometry_shader() -> Result<GeometryShader> {
    Ok(GeometryShader::from_source("\
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
    }  \
    ")?)
}

pub fn create_cube_frag_shader() -> Result<FragmentShader> {
    Ok(FragmentShader::from_source("\
    #version 330 core
    
    in VertexData {
        vec2 position;
        flat vec4 Amount;
        flat bool right;
    } vertex;

    uniform float margin;

    float sampleQuad(vec2 pos) {
        return mix (
            mix(vertex.Amount[0], vertex.Amount[1], pos[0]),
            mix(vertex.Amount[2], vertex.Amount[3], 1 - pos[0]),
            pos[1]
        );
    }

    out vec4 Color;
    
    void main()
    {
        vec2 pos;
        float sample = sampleQuad(vertex.position);
        if (sample > margin) {
            Color = vec4(sample, sample, sample, 1);
        }else {
            Color = vec4(0, 0, 0, 0);
        }
        
    }\
    ")?)
}