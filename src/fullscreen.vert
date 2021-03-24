#version 450

layout(location = 0) out float index;

void main() {
    index = float(gl_InstanceIndex);
    uint id = gl_VertexIndex + gl_InstanceIndex * 3;
    float x = float(((uint(id) + 2u) / 3u)%2u); 
    float y = float(((uint(id) + 1u) / 3u)%2u); 

    gl_Position = vec4(-1.0f + x*2.0f, -1.0f+y*2.0f, 0.0f, 1.0f);
}