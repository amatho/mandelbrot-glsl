#version 150 core
#extension GL_ARB_gpu_shader_fp64 : enable

uniform uvec2 screen_size;
uniform dvec2 center_point;
uniform double pixel_delta;
uniform uint max_iterations;

layout(origin_upper_left) in vec4 gl_FragCoord;

out vec4 color;

double pow2(double x) {
    return x * x;
}

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    double c_re = center_point.x + (pixel_delta * (gl_FragCoord.x - screen_size.x / 2));
    double c_im = center_point.y - (pixel_delta * (gl_FragCoord.y - screen_size.y / 2));
    dvec2 c = dvec2(c_re, c_im);
    dvec2 z = c;

    color = vec4(0.0, 0.0, 0.0, 1.0);
    
    float i;
    for(i = 0; i < max_iterations; i++) {
        z = dvec2(pow2(z.x) - pow2(z.y), 2 * z.x * z.y) + c;

        if (pow2(z.x) + pow2(z.y) > 4) {
            break;
        }
    }

    if (i == max_iterations) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        float val = i / float(max_iterations);
        color = vec4(hsv2rgb(vec3(val, 1.0, 1.0)), 1.0);
    }
}
