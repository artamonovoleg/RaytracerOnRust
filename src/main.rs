mod math;
use math::{Vector3, unit_vector};

#[derive(Copy, Clone)]
pub struct Ray
{
    orig: Vector3,
    dir: Vector3
}

impl Ray
{
    #[allow(dead_code)]
    pub fn new(origin: Vector3, direction: Vector3) -> Self
    {
        Self { orig: origin, dir: direction }
    }

    #[allow(dead_code)]
    pub fn origin(&self) -> Vector3
    {
        self.orig
    }

    #[allow(dead_code)]
    pub fn direction(&self) -> Vector3
    {
        self.dir
    }

    #[allow(dead_code)]
    pub fn at(&self, t : f64) -> Vector3
    {
        self.orig + t * self.dir
    }
}

pub fn ray_color(r : Ray) -> Vector3
{
    let unit_direction: Vector3 = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() 
{
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::origin();
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = 
        origin 
        - horizontal / 2.0 
        - vertical / 2.0 
        - Vector3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {} \n255\n", image_width, image_height);
    for j  in (0 .. (image_height - 1)).rev()
    {
        eprint!("\rScanlines remaining {} ", j);

        for i in 0 .. image_width
        {
            let u  = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let ray = Ray::new(
                origin, 
                lower_left_corner + u * horizontal + v * vertical - origin);
            
            let pixel_color = ray_color(ray);

            write_color(pixel_color);
        }
    }
    eprint!("\nDone.\n");
}

pub fn write_color(v: Vector3)
{
    println!("{} {} {}", 
        (255.999 * v.x()) as i32, 
        (255.999 * v.y()) as i32, 
        (255.999 * v.z()) as i32)
}