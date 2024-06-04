#version 330 core

layout (location = 0) in vec3 iPosition;
layout (location = 1) in vec3 iColor;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

out vec3 Color;

void main()
{
    Color = iColor;
    gl_Position = uProjection * uView * uModel * vec4(iPosition, 1.0);
}
