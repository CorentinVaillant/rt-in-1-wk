use crate::math::{Intervall, Vec3};

pub type Color = Vec3;

pub const WHITE:Color = Color::new(1.0, 1.0, 1.0);
pub const BLACK:Color = Color::ZERO;

#[inline]
pub fn linear_to_gamma(linear_comp: f64) -> f64 {
    if linear_comp > 0. {
        f64::sqrt(linear_comp)
    } else {
        0.
    }
}

const COLOR_INTERVALL: Intervall = Intervall::new(0., 0.999);
pub fn write_color_to_pixel_buff(pixel_buff: &mut [[u8; 3]], pos: usize, color: Color) {

    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());

    // Clamp and convert to byte
    pixel_buff[pos] = [
        (256.0 * COLOR_INTERVALL.clamp(r)) as u8,
        (256.0 * COLOR_INTERVALL.clamp(g)) as u8,
        (256.0 * COLOR_INTERVALL.clamp(b)) as u8,
    ];
}
