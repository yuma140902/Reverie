#version 330 core

struct Material {
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in float Alpha;
in vec3 FragPosition;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D uScreenTexture;
uniform vec3 uViewPosition;
uniform Material uMaterial;
uniform Light uLight;

void main()
{
    vec3 texRGB = texture(uScreenTexture, TexCoords).rgb;

    /* ambient*/
    vec3 ambient = uLight.ambient * texRGB;

    /* diffuse */
    float diffuseScala = dot(normalize(Normal), normalize(-uLight.direction));
    vec3 diffuse = uLight.diffuse * max(diffuseScala, 0.0) * texRGB;

    /* specular */
    vec3 viewDirection = normalize(uViewPosition - FragPosition);
    vec3 reflectDirection = reflect(normalize(uLight.direction), normalize(Normal));
    float specularScala = dot(viewDirection, reflectDirection);
    vec3 specular = uLight.specular * pow(max(specularScala, 0.0), uMaterial.shininess) * uMaterial.specular;

    vec3 result = ambient + diffuse + specular;

    // gl_FragColor = vec4(FragPosition, 1.0);
    gl_FragColor = vec4(result, Alpha);
}
