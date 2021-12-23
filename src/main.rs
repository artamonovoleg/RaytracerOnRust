mod math;
use math::Vector3;

fn main() 
{
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {} \n255\n", image_width, image_height);
    for j  in (0 .. (image_height - 1)).rev()
    {
        eprint!("\rScanlines remaining {} ", j);

        for i in 0 .. image_width
        {
            let r : f64 = i as f64 / (image_width as f64 - 1.0);
            let g : f64 = j as f64 / (image_height as f64 - 1.0);
            let b : f64 = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\nDone.\n");
}
