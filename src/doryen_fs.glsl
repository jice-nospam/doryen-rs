precision mediump float;
uniform sampler2D uFont;
uniform sampler2D uAscii;
uniform sampler2D uFront;
uniform sampler2D uBack;
uniform float uFontCharsPerLine;
uniform vec2 uFontCoef;
uniform vec2 uTermCoef;

in vec2 vTextureCoord;
out vec4 FragColor;
void main(){
    vec2 pixPos = fract(vTextureCoord) * uFontCoef;
    vec2 address = floor(vTextureCoord) * uTermCoef + vec2(0.001, 0.001);
    vec4 ascii_vec = texture(uAscii, address);
    float ascii_code = (ascii_vec.r * 256.0) + (ascii_vec.g * 256.0 * 256.0);
    vec4 foreground = texture(uFront, address);
    vec4 background = texture(uBack, address);
    vec2 tchar = vec2(mod(floor(ascii_code), floor(uFontCharsPerLine)), floor(ascii_code / uFontCharsPerLine));
    vec4 font_color = texture(uFont, tchar * uFontCoef + pixPos);
    FragColor=font_color.a * foreground * vec4(font_color.rgb,1.0) + (1.0 - font_color.a) * background;
}
