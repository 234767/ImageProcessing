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

void main() {
    ivec2 max_size = imageSize(inImage);
    ivec2 ID = ivec2(gl_GlobalInvocationID.xy);

    vec3 to_write = vec3(0.0,0.0,0.0);

    COLLECT_PIXELS (
             if (pixel.r > to_write.r)
                 to_write.r = pixel.r;
            if (pixel.g > to_write.g)
                to_write.g = pixel.g;
            if (pixel.b > to_write.b)
                to_write.b = pixel.b;
    )

    imageStore(outImage, ID, vec4(to_write,1.0));
}