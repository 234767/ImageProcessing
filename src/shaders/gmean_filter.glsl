#version 450
#include "shader_macros.h"

layout (local_size_x = 16, local_size_y = 16, local_size_z = 1) in;


layout (push_constant) uniform PushConstantData {
    uint x_radius;
    uint y_radius;
} dat;

layout (set = 0, binding = 0, rgba8) uniform readonly image2D inImage;
layout (set = 0, binding = 1, rgba8) uniform writeonly image2D outImage;

uint arraySize;
float sum;

float mean() {
    return sum / arraySize;
}

void main() {
    ivec2 max_size = imageSize(inImage);
    ivec2 ID = ivec2(gl_GlobalInvocationID.xy);

    vec3 to_write = vec3(0.0, 0.0, 0.0);

    arraySize = 0;
    sum = 0.f;
    COLLECT_PIXELS (
        sum += pixel.r;
        arraySize++;
        )
    to_write.r = mean();

    arraySize = 0;
    sum = 0.f;
    COLLECT_PIXELS (
        sum += pixel.g;
        arraySize++;
        )
    to_write.g = mean();

    arraySize = 0;
    sum = 0.f;
    COLLECT_PIXELS (
        sum += pixel.b;
        arraySize++;
        )
    to_write.b = mean();

    imageStore(outImage, ID, vec4(to_write, 1.0));
}