#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 center;
varying lowp vec2 uv;
varying lowp vec2 uv_screen;
uniform mat4 Model;
uniform mat4 Projection;
uniform vec2 Center;
uniform vec2 ray;
uniform float time;
void main() {
    vec3 p=position;
    vec4 res =  Projection* Model * vec4(p, 1);
    vec4 c = Projection * Model * vec4(Center, 0, 1);
    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    
    center = c.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;
    gl_Position = res;
}