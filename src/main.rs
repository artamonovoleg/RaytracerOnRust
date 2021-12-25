use std::fs::File;
use std::io::Write;
mod math;
use math::Vector3;

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

pub struct HitRecord {
    point: Vector3,
    normal: Vector3,
    distance: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vector3::origin(),
            normal: Vector3::origin(),
            distance: std::f64::MAX,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vector3) {
        self.front_face = math::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

trait Hit {
    fn hit(
        &self,
        ray: Ray,
        min_distance: f64,
        max_distance: f64,
        hit_record: &mut HitRecord,
    ) -> bool;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    fn new() -> Self {
        Self {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 0.0,
        }
    }
}

impl Hit for Sphere {
    fn hit(
        &self,
        ray: Ray,
        min_distance: f64,
        max_distance: f64,
        hit_record: &mut HitRecord,
    ) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = math::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        let mut result = false;
        if discriminant < 0.0 {
            result = false;
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < min_distance || max_distance < root {
                root = -half_b + sqrtd / a;
                if root < min_distance || max_distance < root {
                    result = false;
                }
            } else {
                hit_record.distance = root;
                hit_record.point = ray.at(hit_record.distance);
                let outward_normal = (hit_record.point - self.center) / self.radius;
                hit_record.set_face_normal(ray, outward_normal);
                result = true;
            }
        }

        return result;
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const BPP: i32 = 3;
    const IMAGE_DATA_SIZE: usize = (IMAGE_WIDTH * IMAGE_HEIGHT * BPP) as usize;

    let mut pixels: [u8; IMAGE_DATA_SIZE] = [0; IMAGE_DATA_SIZE];

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Vector3::origin();
    let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    // Scene
    let mut spheres: [Sphere; 2] = [Sphere::new(); 2];
    spheres[0].radius = 0.5;
    spheres[0].center = Vector3::new(0.0, 0.0, -1.0);

    spheres[1].radius = 0.3;
    spheres[1].center = Vector3::new(0.3, 0.0, -0.8);

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = (IMAGE_HEIGHT - y) as f64 / (IMAGE_HEIGHT - 1) as f64;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let idx = get_pixel_index(x, y, IMAGE_WIDTH, BPP);
            let color = ray_color(ray, &spheres);
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

pub fn ray_color(r: Ray, spheres: &[Sphere]) -> Vector3 {
    let mut result_record = HitRecord::new();
    let mut hit_anything = false;
    for i in 0..spheres.len() {
        let mut hit_record = HitRecord::new();

        if spheres[i].hit(r, 0.0, std::f64::MAX, &mut hit_record) {
            hit_anything = true;
            if hit_record.distance < result_record.distance {
                result_record = hit_record;
            }
        }
    }
    
    if hit_anything {
        return 0.5 * (result_record.normal + Vector3::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vector3 = math::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
}
