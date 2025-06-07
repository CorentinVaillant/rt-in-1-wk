use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct PixelBuff{
    pub(super) pixels : Vec<[u8;3]>,

    pub(super) heigth :usize,
    pub(super) width  :usize,
}

impl PixelBuff {
    pub fn empty()->Self{
        Self { pixels: Vec::new(), heigth: 0, width: 0 }
    }

    pub(super) fn zeroed(heigth:usize, width:usize)->Self{
        let pixels:Vec<_> = (0..(heigth*width + 1)).map(|_|[0_u8;3]).collect();
        Self{
            pixels,

            heigth,
            width
        }
    }
}

impl PixelBuff {
    pub fn write_into_bmp(&self,file:&mut File){
        let mut s_file = String::with_capacity(self.heigth * self.width + 20);
        let header = format!("P3\n{} {}\n255\n",self.width,self.heigth);
        s_file.push_str(&header);
        let mut s_pixel = String::with_capacity(12);
        for pixel in self.pixels.iter(){
            s_pixel.clear();
            std::fmt::Write::write_str(&mut s_pixel, &format!("{} {} {}\n",pixel[0],pixel[1],pixel[2]))
                .expect("smt went wrong");
            s_file.push_str(&s_pixel);
        }

        file.write(s_file.as_bytes()).unwrap();
    }
}