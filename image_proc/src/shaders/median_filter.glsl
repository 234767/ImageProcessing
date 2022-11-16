#version 450
#include "shader_macros.h"

layout (local_size_x = 16, local_size_y = 16, local_size_z = 1) in;


layout (push_constant) uniform PushConstantData {
    uint x_radius;
    uint y_radius;
} dat;

layout (set = 0, binding = 0, rgba8) uniform readonly image2D inImage;
layout (set = 0, binding = 1, rgba8) uniform writeonly image2D outImage;

uint num_elements[3];
uint luminosity_buckets[3][256];

void clear_values() {
    for (int channel = 0; channel < 3; channel++) {
        num_elements[channel] = 0;
        for (int i = 0; i < 256; i++) {
            luminosity_buckets[channel][i] = 0;
        }
    }
    
}

uint get_median(uint array[256], uint total_elements) {
    uint median_index = total_elements / 2;
    uint current_index = 0;
    for (int luminosity = 0; luminosity < 256; luminosity++) {
        current_index += array[luminosity];
        if (current_index > median_index) {
            return uint(luminosity);
        }
    }
    return 255;
}

uint float_to_uint(float value) {
    uint new_value = uint(value * 255.f + 0.1f);
    return new_value <= 255 ? new_value : 255;
}

float uint_to_float(uint value) {
    return float(value) / 255.f;
}

void main() {
    ivec2 max_size = imageSize(inImage);
    ivec2 ID = ivec2(gl_GlobalInvocationID.xy);

    vec3 to_write = vec3(0.0,0.0,0.0);

    clear_values();
    COLLECT_PIXELS (
        uint luminosity;
        luminosity = float_to_uint(pixel.r);
        luminosity_buckets[0][luminosity]++;
        num_elements[0]++;

        luminosity = float_to_uint(pixel.g);
        luminosity_buckets[1][luminosity]++;
        num_elements[1]++;

        luminosity = float_to_uint(pixel.b);
        luminosity_buckets[2][luminosity]++;
        num_elements[2]++;
        )
    to_write.r = uint_to_float(get_median(luminosity_buckets[0],num_elements[0]));
    to_write.g = uint_to_float(get_median(luminosity_buckets[1],num_elements[1]));
    to_write.b = uint_to_float(get_median(luminosity_buckets[2],num_elements[2]));

    imageStore(outImage, ID, vec4(to_write,1.0));
}
