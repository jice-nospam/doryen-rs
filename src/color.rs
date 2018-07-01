pub type Color = (u8, u8, u8, u8);

pub fn color_blend(c1: &Color, c2: &Color, alpha: f32) -> Color {
    (
        (((1.0 - alpha) * c1.0 as f32) + alpha * (c2.0 as f32)) as u8,
        (((1.0 - alpha) * c1.1 as f32) + alpha * (c2.1 as f32)) as u8,
        (((1.0 - alpha) * c1.2 as f32) + alpha * (c2.2 as f32)) as u8,
        255,
    )
}

pub fn color_dist(c1: &Color, c2: &Color) -> i32 {
    let dr = c1.0 as i32 - c2.0 as i32;
    let dg = c1.1 as i32 - c2.1 as i32;
    let db = c1.2 as i32 - c2.2 as i32;
    dr * dr + dg * dg + db * db
}
