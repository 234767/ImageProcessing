#version 450

layout (local_size_x = 16, local_size_y = 16, local_size_z = 1) in;

layout (push_constant) uniform PushConstantData {
        float[9] mask;
} dat;

layout (set = 0, binding = 0, rgba8) uniform readonly image2D inImage;
layout (set = 0, binding = 1, rgba8) uniform writeonly image2D outImage;

ivec2 kpos(int index){
    return ivec2[9](
        ivec2(-1,-1), ivec2(0,-1), ivec2(1,-1),
        ivec2(-1, 0), ivec2(0,0), ivec2(1,0),
        ivec2(-1,1), ivec2(0,1), ivec2(1,1)
    )[index];
}

mat3[3] region3x3(ivec2 uv) {
    vec4[9] region;
    for ( int i = 0; i < 9; ++i ) {
        region[i] = imageLoad(inImage, uv + kpos(i));
    }

    // Create 3x3 region with 3 color channels (red, green, blue)
    mat3[3] mRegion;

    for (int i = 0; i < 3; i++)
        mRegion[i] = mat3(
            region[0][i], region[1][i], region[2][i],
            region[3][i], region[4][i], region[5][i],
            region[6][i], region[7][i], region[8][i]
        );

    return mRegion;
}

vec3 convolution(mat3 mask, ivec2 uv)
{
    vec3 fragment;

    // Extract a 3x3 region centered in uv
    mat3[3] region = region3x3(uv);

    // for each color channel of region
    for (int i = 0; i < 3; i++)
    {
        // get region channel
        mat3 rc = region[i];
        // component wise multiplication of kernel by region channel
        mat3 c = matrixCompMult(mask, rc);
        // add each component of matrix
        float r = c[0][0] + c[1][0] + c[2][0]
                  + c[0][1] + c[1][1] + c[2][1]
                  + c[0][2] + c[1][2] + c[2][2];

        // for fragment at channel i, set result
        fragment[i] = r;
    }

    return fragment;
}

void main() {
    ivec2 max_size = imageSize(inImage);
    ivec2 ID = ivec2(gl_GlobalInvocationID.xy);
//    if (ID.x == 0 || ID.x == max_size.x - 1 || ID.y == 0 || ID.y == max_size.y - 1){
//        imageStore(outImage, ID, imageLoad(inImage, ID));
//        return;
//    }

    // has to be this ugly beacuse there was problem with loading it as a push constant
    mat3 mask = mat3(dat.mask[0],dat.mask[1],dat.mask[2],dat.mask[3],dat.mask[4],dat.mask[5],dat.mask[6],dat.mask[7],dat.mask[8]);
    vec3 newPixel = convolution(mask, ID);
    imageStore(outImage, ID, vec4(newPixel,1.0));
}