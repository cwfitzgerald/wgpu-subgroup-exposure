#version 450

#extension GL_KHR_shader_subgroup_arithmetic : require

layout(set = 0, binding = 0) writeonly buffer Buffer {
    float count[];
};

void main() {
    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
    count[gl_InstanceIndex] = subgroupAdd(1.0);
}
