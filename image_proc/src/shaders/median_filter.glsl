#version 450
#include "shader_macros.h"
#define MAX_SIZE 400

layout (local_size_x = 16, local_size_y = 16, local_size_z = 1) in;


layout (push_constant) uniform PushConstantData {
    uint x_radius;
    uint y_radius;
} dat;

layout (set = 0, binding = 0, rgba8) uniform readonly image2D inImage;
layout (set = 0, binding = 1, rgba8) uniform writeonly image2D outImage;

uint numElements;
float luminosities[MAX_SIZE];

void sort() {
    for ( int k=1; k < int( numElements); k++)
    {
        float temp = luminosities[k];
        int j = k-1;
        while (j>=0 && temp <= luminosities[j])
        {
            luminosities[j+1] = luminosities[j];
            j = j-1;
        }
        luminosities[j+1] = temp;
    }
}
void main() {
    ivec2 max_size = imageSize(inImage);
    ivec2 ID = ivec2(gl_GlobalInvocationID.xy);

    vec3 to_write = vec3(0.0,0.0,0.0);

    numElements = 0;
    COLLECT_PIXELS (
        luminosities[numElements] = pixel.r;
        numElements++;
        )
    sort();
    to_write.r = luminosities[numElements / 2];

    numElements = 0;
    COLLECT_PIXELS (
        luminosities[numElements] = pixel.g;
        numElements++;
        )
    sort();
    to_write.g = luminosities[numElements / 2];

    numElements = 0;
    COLLECT_PIXELS (
        luminosities[numElements] = pixel.b;
        numElements++;
        )
    sort();
    to_write.b = luminosities[numElements / 2];

    imageStore(outImage, ID, vec4(to_write,1.0));
}