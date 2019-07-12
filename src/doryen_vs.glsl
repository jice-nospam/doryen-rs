in vec2 aVertexPosition;
in vec2 aTextureCoord;
out vec2 vTextureCoord;
uniform vec2 uTermSize;
void main(void) {
    // vertex position from (-1,-1) to (1,1)
    gl_Position = vec4(aVertexPosition.xy, 0.0, 1.0);
    // texture coordinates from (0,0) to (console_width,console_height)
    vTextureCoord = aTextureCoord * uTermSize;
}
