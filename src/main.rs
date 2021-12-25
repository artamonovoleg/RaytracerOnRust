use std::fs::File;
use std::io::Write;
mod math;
use math::{unit_vector, Vector3};

#[derive(Copy, Clone)]
pub struct Ray {
    orig: Vector3,
    dir: Vector3,
}

impl Ray {
    #[allow(dead_code)]
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    #[allow(dead_code)]
    pub fn origin(&self) -> Vector3 {
        self.orig
    }

    #[allow(dead_code)]
    pub fn direction(&self) -> Vector3 {
        self.dir
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Vector3 {
        self.orig + t * self.dir
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const BPP: i32 = 3;
    const IMAGE_DATA_SIZE: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * BPP) as usize;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vector3::origin();
    let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut pixels: [u8; IMAGE_DATA_SIZE] = [0; IMAGE_DATA_SIZE];

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = (IMAGE_HEIGHT - y) as f64 / (IMAGE_HEIGHT - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let idx = get_pixel_index(x, y, IMAGE_WIDTH, BPP);
            let color = ray_color(ray);
            set_pixel(idx, color, &mut pixels);
        }
    }

    write_image(IMAGE_WIDTH, IMAGE_HEIGHT, &pixels).unwrap();
}

fn get_pixel_index(x: i32, y: i32, width: i32, bpp: i32) -> usize {
    (x * bpp + y * bpp * width) as usize
}

fn set_pixel(index: usize, color: Vector3, pixels: &mut [u8]) {
    pixels[index] = (255.999 * color.x()) as u8;
    pixels[index + 1] = (255.999 * color.y()) as u8;
    pixels[index + 2] = (255.999 * color.z()) as u8;
}

fn write_image(width: i32, height: i32, pixels: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("image.ppm")?;
    write!(file, "P6\n{} {} \n255\n", width, height)?;
    file.write(pixels)?;
    Ok(())
}

pub fn ray_color(r: Ray) -> Vector3 {
    let mut t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = unit_vector(r.at(t) - Vector3::new(0.0, 0.0, -1.0));
        return 0.5 * Vector3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    } else {
        let unit_direction: Vector3 = math::unit_vector(r.direction());
        t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}

pub fn hit_sphere(center: Vector3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = math::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
