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
    
}