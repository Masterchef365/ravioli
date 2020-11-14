#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragCoord;

layout(binding = 1) uniform Animation {
    float u_time;
};

layout(location = 0) out vec4 outColor;

const vec2 u_resolution = vec2(1.);
#define gl_FragCoord fragCoord
#define gl_FragColor outColor

// #################################################################

void main() {
    vec2 st = gl_FragCoord.xy/u_resolution.xy;
    vec3 color = vec3(st, fragCoord.z) / 1.;
    gl_FragColor = vec4(color, 1.);
}
 
