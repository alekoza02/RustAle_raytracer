mod camera;
mod matematica;

use camera::camera::Camera;

const W : i32 = 4;
const H : i32 = 4;

fn main() {
    let mut camera = Camera::new([0.,0.,30.], [0.,0.,-1.], [0.,1.,0.], [1.,0.,0.], 60.);

    for x in 0..W{
        for y in 0..H {
            camera.dir_pix = camera.genera_direzione(&(x as f64), &(y as f64), &(W as f64), &(H as f64));
            println!("{}, {}, {}", camera.dir_pix[0], camera.dir_pix[1], camera.dir_pix[2])
        }
        println!("\n")
    }

}
