precision mediump float;
uniform sampler2D uDiffuse;
in vec2 vTextureCoord;
out vec4 FragColor;
void main(){
    FragColor = texture(uDiffuse, vTextureCoord);
}
