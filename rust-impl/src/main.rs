use std::{fs::File, rc::Rc};

use rand::random;

use crate::{color::Color, geometry::Sphere, light::{hittable_list::HittableList, material::{DielectricMat, LambertianMat, Material, MetalMat}}, math::Point3, render::{pixel_buff::PixelBuff, Camera}};

pub mod color;
pub mod light;
pub mod math;
pub mod render;
pub mod geometry;


fn main() {
    println!("Starting [App]");

    let mut pixel_buff = PixelBuff::empty();

    let mut world = HittableList::empty();

    let ground_mat = Rc::new(LambertianMat::new(Color::new(0.5,0.5,0.5)));
    world.push(Rc::new(Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_mat)));


    for a in -11..11{
        for b in -11..11{
            let (a,b) = (a as f64, b as f64);
            let choose_mat = random::<f64>()%1.;
            let center = Point3::new(a + 0.9*(random::<f64>()%1.), 0.2, b + 0.9*(random::<f64>()%1.));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let spehre_mat:Rc<dyn Material> = if choose_mat < 0.8{
                    //Diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(LambertianMat::new(albedo))
                }else if choose_mat < 0.95{
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random::<f64>()%0.5;
                    Rc::new(MetalMat::new(albedo, fuzz))
                }else{
                    Rc::new(DielectricMat::new(1.5))
                };
            
            world.push(Rc::new(Sphere::new(center, 0.2, spehre_mat)));
            }
        }
    }

    let mat1 = Rc::new(DielectricMat::new(1.5));
    world.push(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Rc::new(LambertianMat::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Rc::new(MetalMat::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    let mut cam = Camera::default();


    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1080;
    cam.samples_per_pixel = 20;
    cam.max_depth         = 5;

    cam.vfov     = 20.;
    cam.lookfrom = Point3::new(13.,2.,3.);
    cam.lookat   = Point3::new(0.,0.,0.);
    cam.vup      = Point3::new(0.,1.,0.);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.render(&world,&mut pixel_buff);

    let path = "./img.bmp";

    let mut img = match File::create_new(path) {
        Ok(f) => f,
        Err(_) => File::options().write(true).truncate(true).open(path).unwrap(),
    }; 

    // println!("{:?}",pixel_buff);

    pixel_buff.write_into_bmp(&mut img);

    drop(img);
    println!("Done")
}
