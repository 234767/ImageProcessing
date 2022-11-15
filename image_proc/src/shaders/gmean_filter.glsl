#version 450

#include "shader_macros.h"

layout (local_size_x = 16, local_size_y = 16, local_size_z = 1
)
in;


layout (push_constant)
uniform PushConstantData{ uint x_radius;
uint y_radius; }
dat;

layout (set = 0, binding = 0, rgba8
)
uniform readonly
image2D inImage;
layout (set = 0, binding = 1, rgba8
)
uniform writeonly
image2D outImage;


float root( float radicand, uint index ) {
    return pow( radicand, ( 1.f / float( index )));
}

void main() {
    ivec2 max_size = imageSize( inImage );
    ivec2 ID = ivec2( gl_GlobalInvocationID.xy );

    vec3 to_write = vec3( 0.0, 0.0, 0.0 );

    vec3 products = vec3( 1.0, 1.0, 1.0 );
    uint numElements = 0;

    COLLECT_PIXELS (
        products.r *= pixel.r;
        products.g *= pixel.g;
        products.b *= pixel.b;
        numElements++;
    )
    to_write.r = root(products.r, numElements);
    to_write.g = root(products.g, numElements);
    to_write.b = root(products.b, numElements);

    imageStore( outImage, ID, vec4( to_write, 1.0 ));
}