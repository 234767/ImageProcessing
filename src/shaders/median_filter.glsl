#version 450

#define MAX_SIZE 400

layout (local_size_x = 16, local_size_y = 16, local_size_z = 1) in;


layout (push_constant) uniform PushConstantData {
    uint x_radius;
    uint y_radius;
} dat;

layout (set = 0, binding = 0, rgba8) uniform readonly image2D inImage;
layout (set = 0, binding = 1, rgba8) uniform writeonly image2D outImage;

uint arraySize;
float luminosities[MAX_SIZE];

void sort() {
    for (int k=1; k < int(arraySize); k++)
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

    arraySize = 0;
    for (int x = int(ID.x) - int(dat.x_radius); x <= int(ID.x) + int(dat.x_radius); x++ ) {
        for (int y = int(ID.y) - int(dat.y_radius); y <= int(ID.y) + int(dat.y_radius); y++) {
            if (x > 0 && y > 0 && x < max_size.x && y < max_size.y && arraySize < MAX_SIZE) {
                vec4 pixel = imageLoad(inImage, ivec2(x,y));
                luminosities[arraySize] = pixel.r;
                arraySize++;
            }
        }
    }
    sort();
    to_write.r = luminosities[arraySize/2];

    arraySize = 0;
    for (int x = int(ID.x) - int(dat.x_radius); x <= int(ID.x) + int(dat.x_radius); x++ ) {
        for (int y = int(ID.y) - int(dat.y_radius); y <= int(ID.y) + int(dat.y_radius); y++) {
            if (x > 0 && y > 0 && x < max_size.x && y < max_size.y && arraySize < MAX_SIZE) {
                vec4 pixel = imageLoad(inImage, ivec2(x,y));
                luminosities[arraySize] = pixel.g;
                arraySize++;
            }
        }
    }
    sort();
    to_write.g = luminosities[arraySize/2];

    arraySize = 0;
    for (int x = int(ID.x) - int(dat.x_radius); x <= int(ID.x) + int(dat.x_radius); x++ ) {
        for (int y = int(ID.y) - int(dat.y_radius); y <= int(ID.y) + int(dat.y_radius); y++) {
            if (x > 0 && y > 0 && x < max_size.x && y < max_size.y && arraySize < MAX_SIZE) {
                vec4 pixel = imageLoad(inImage, ivec2(x,y));
                luminosities[arraySize] = pixel.b;
                arraySize++;
            }
        }
    }
    sort();
    to_write.b = luminosities[arraySize/2];

    imageStore(outImage, ID, vec4(to_write,1.0));
}
