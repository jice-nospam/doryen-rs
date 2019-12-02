precision mediump float;
uniform sampler2D uFont; // font texture
uniform sampler2D uAscii; // console ascii code
uniform sampler2D uFront; // console foreground color
uniform sampler2D uBack; // console background color
// used to convert ascii code into x,y character pos inside the font
uniform float uFontCharsPerLine;
// converts character pos inside font (0,0) - (16,16) into texture coord (0,0) - (1,1)
uniform vec2 uFontCoef;
// converts cell pos inside console (0,0) - (console_width,console_height) into texture coord (0,0) - (1,1)
// the size of the console texture is not console_width x console_height but the closest power of 2 values
uniform vec2 uTermCoef;

in vec2 vTextureCoord;
out vec4 FragColor;
void main(){
    // address = coordinate in the console textures (front, back, ascii) between (0,0) and (1,1)
    vec2 address = floor(vTextureCoord) * uTermCoef + vec2(0.001, 0.001);
    // get the u32 ascii code from the ascii texture
    vec4 ascii_vec = texture(uAscii, address);
    float ascii_code = (ascii_vec.r * 255.0) + (ascii_vec.g * 255.0 * 256.0);
    // get the foreground and background colors
    vec4 foreground = texture(uFront, address);
    vec4 background = texture(uBack, address);
    // get coordinate of the glyph in the font texture for the ascii character
    vec2 tchar = vec2(mod(floor(ascii_code), floor(uFontCharsPerLine)), floor(ascii_code / uFontCharsPerLine));
    // where are we inside the cell / glyph
    vec2 pixPos = fract(vTextureCoord) * uFontCoef;
    vec4 font_color = texture(uFont, tchar * uFontCoef + pixPos);
    FragColor=font_color.a * foreground * vec4(font_color.rgb,1.0) + (1.0 - font_color.a) * background;
}
