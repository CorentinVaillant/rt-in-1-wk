use crate::math::{Intervall, Vec3};

pub type Color = Vec3;

#[inline]
pub fn linear_to_gamma(linear_comp:f64)->f64{
    if linear_comp >0.{
        f64::sqrt(linear_comp)
    }else{
        0.
    }
}

const COLOR_INTERVALL :Intervall = Intervall::new(0., 0.999);
pub fn write_color_to_pixel_buff(pixel_buff :&mut [[u8;3]],pos:usize, color:Color){
    let mut r= color.x();
    let mut g= color.y();
    let mut b= color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let rbyte = (256. * COLOR_INTERVALL.clamp(r)) as u8;
    let gbyte = (256. * COLOR_INTERVALL.clamp(g)) as u8;
    let bbyte = (256. * COLOR_INTERVALL.clamp(b)) as u8;

    pixel_buff[pos] = [rbyte, gbyte, bbyte]
}