#ifndef IMAGEPROCESSING_SHADER_MACROS_H
#define IMAGEPROCESSING_SHADER_MACROS_H

#define COLLECT_PIXELS( body ) for (int x = int(ID.x) - int(dat.x_radius); x <= int(ID.x) + int(dat.x_radius); x++ ) { \
        for (int y = int(ID.y) - int(dat.y_radius); y <= int(ID.y) + int(dat.y_radius); y++) { \
            if (x > 0 && y > 0 && x < max_size.x && y < max_size.y) { \
                vec4 pixel = imageLoad(inImage, ivec2(x,y)); \
                body \
        }}}

#endif //IMAGEPROCESSING_SHADER_MACROS_H
