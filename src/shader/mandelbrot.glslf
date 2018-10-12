#version 150 core
#extension GL_ARB_gpu_shader_fp64 : enable

layout(origin_upper_left) in vec4 gl_FragCoord;
out vec4 Target0;

uniform Locals {
    double i_ScreenWidth;
    double i_ScreenHeight;
    double i_MaxIterations;
    double i_PixelDelta;
    double i_CenterRe;
    double i_CenterIm;
};

double pow2(double x) {
    return x * x;
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    uint max_iter = uint(i_MaxIterations);
    double c_re = i_CenterRe + (i_PixelDelta * (gl_FragCoord.x - i_ScreenWidth / 2));
    double c_im = i_CenterIm - (i_PixelDelta * (gl_FragCoord.y - i_ScreenHeight / 2));
    dvec2 c = dvec2(c_re, c_im);
    dvec2 z = c;
    
    float i;
    for(i = 0; i < max_iter; i++) {
        z = dvec2(pow2(z.x) - pow2(z.y), 2 * z.x * z.y) + c;

        if (pow2(z.x) + pow2(z.y) > 4) {
            break;
        }
    }

    if (i == max_iter) {
        Target0 = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        float val = i / float(max_iter);
        Target0 = vec4(hsv2rgb(vec3(val, 1.0, 1.0)), 1.0);
    }
}
