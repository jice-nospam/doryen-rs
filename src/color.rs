pub type Color = (u8, u8, u8, u8);

pub fn color_blend(c1: Color, c2: Color, alpha: f32) -> Color {
    let alpha = alpha * c2.3 as f32 / 255.0;
    (
        ((1.0 - alpha) * f32::from(c1.0) + alpha * f32::from(c2.0)) as u8,
        ((1.0 - alpha) * f32::from(c1.1) + alpha * f32::from(c2.1)) as u8,
        ((1.0 - alpha) * f32::from(c1.2) + alpha * f32::from(c2.2)) as u8,
        255,
    )
}

pub fn color_scale(c: Color, coef: f32) -> Color {
    (
        (f32::from(c.0) * coef).min(255.0) as u8,
        (f32::from(c.1) * coef).min(255.0) as u8,
        (f32::from(c.2) * coef).min(255.0) as u8,
        c.3,
    )
}

pub fn color_mul(c1: Color, c2: Color) -> Color {
    (
        (f32::from(c1.0) * f32::from(c2.0) / 255.0) as u8,
        (f32::from(c1.1) * f32::from(c2.1) / 255.0) as u8,
        (f32::from(c1.2) * f32::from(c2.2) / 255.0) as u8,
        255,
    )
}

pub fn color_add(c1: Color, c2: Color) -> Color {
    (
        (0.5 * f32::from(c1.0) + 0.5 * f32::from(c2.0)) as u8,
        (0.5 * f32::from(c1.1) + 0.5 * f32::from(c2.1)) as u8,
        (0.5 * f32::from(c1.2) + 0.5 * f32::from(c2.2)) as u8,
        (0.5 * f32::from(c1.3) + 0.5 * f32::from(c2.3)) as u8,
    )
}

pub fn color_dist(c1: Color, c2: Color) -> i32 {
    let dr = i32::from(c1.0) - i32::from(c2.0);
    let dg = i32::from(c1.1) - i32::from(c2.1);
    let db = i32::from(c1.2) - i32::from(c2.2);
    dr * dr + dg * dg + db * db
}
