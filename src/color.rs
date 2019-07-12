pub type Color = (u8, u8, u8, u8);

pub fn color_blend(c1: &Color, c2: &Color, alpha: f32) -> Color {
    (
        ((1.0 - alpha) * f32::from(c1.0) + alpha * f32::from(c2.0)) as u8,
        ((1.0 - alpha) * f32::from(c1.1) + alpha * f32::from(c2.1)) as u8,
        ((1.0 - alpha) * f32::from(c1.2) + alpha * f32::from(c2.2)) as u8,
        255,
    )
}

pub fn color_dist(c1: &Color, c2: &Color) -> i32 {
    let dr = i32::from(c1.0) - i32::from(c2.0);
    let dg = i32::from(c1.1) - i32::from(c2.1);
    let db = i32::from(c1.2) - i32::from(c2.2);
    dr * dr + dg * dg + db * db
}
