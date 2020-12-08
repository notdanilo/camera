#version 450

layout(location = 0) uniform sampler2D camera;
layout(location = 1) uniform vec2 resolution;

vec4 capture(vec2 uv) {
    return texture(camera, uv * vec2(-1.0));
}

void main() {
    vec2 uv = gl_FragCoord.xy / resolution;
    gl_FragColor = capture(uv);
}