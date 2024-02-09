mod camera;
mod matematica;
mod utils;

use std::f64::consts::PI;

use camera::camera::Camera;
use utils::file::write_ppm;

const W : i32 = 540;
const H : i32 = 540;


fn main() {
    let mut pixels :  [u8; (W*H*3) as usize] = [0; (W*H*3) as usize];
    let mut camera = Camera::new([0.,0.,30.], [0.,0.,-1.], [0.,1.,0.], [1.,0.,0.], PI / 3.0);

    for x in 0..W{
        for y in 0..H {
            camera.dir_pix = camera.genera_direzione(&(x as f64), &(y as f64), &(W as f64), &(H as f64));
            for i in 0..3 {
                pixels[((W * y + x) * 3 + i) as usize] = match i {
                    0 => (camera.dir_pix[0].abs() * 255.0) as u8,
                    1 => (camera.dir_pix[1].abs() * 255.0) as u8,
                    2 => 255,
                    _ => unreachable!()
                }
            } 
        }
    }

    let _ = write_ppm("OUTPUT/output_rust.ppm", &pixels, W, H, 255);

}
