#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;

layout(binding = 1) uniform Animation {
    float time;
};

layout(location = 0) out vec4 outColor;

vec2 position_fg = fragColor.xy;

const float pi = 3.14159274;

int seed = 0;

float rand(vec2 co){
    return fract(sin(dot(co.xy, vec2(12.9898,78.233))) * 43758.5453);
}

float quantize(float t, int s) {
    return int(t * float(s)) / float(s);
}

void main() {
    const vec3 magenta = vec3(255, 0, 172) / 255.0;
    const vec3 blue = vec3(0, 154, 255) / 255.0;

    float cell_y = quantize(position_fg.y - 1.0, 30);
    float mult = rand(vec2(cell_y + 3.321321, 0.0)); 
    float cell_x = quantize(position_fg.x + (time * mult), int(mult * 50.0));

    float gate = float(rand(vec2(cell_x, cell_y)) < 0.2);
    float ratio = rand(vec2(cell_x + 0.324, cell_y));
    vec3 color = mix(magenta, blue, ratio);
    outColor = vec4(color * gate, 1.0);
}
