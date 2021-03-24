#version 450

#extension GL_KHR_shader_subgroup_arithmetic : require

layout(location = 0) in float index;

layout(set = 0, binding = 0) writeonly buffer Buffer {
    float count[];
};

void main() {
    count[uint(index)] = subgroupAdd(index);
}
