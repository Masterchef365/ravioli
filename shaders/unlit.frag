#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;

layout(binding = 1) uniform Animation {
    float u_time;
};

layout(location = 0) out vec4 outColor;

void main() {
    vec3 color = vec3(cos(fragColor.xy * 10. + anim), 0.);
    outColor = vec4(color, 1.0);
}
