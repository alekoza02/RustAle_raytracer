mod camera;
mod utils;
mod geometria;
mod algoritmi;

use std::f64::consts::PI;

use std::io::{self};
use std::time::Instant;

use algoritmi::collisioni::test_collisione;
use camera::camera::Camera;
use geometria::oggetti::Scena;
use utils::file::{write_ppm, Vettore};

const W : i32 = 1080;
const H : i32 = 1080;
const SAMPLES : i32 = 128;


fn main() -> io::Result<()> {

    let start_time = Instant::now();
    let mut pixels: Vec<u8> = vec![0; (W * H * 3) as usize];
    let mut camera = Camera::new(Vettore::new(0.,0.,30.), Vettore::new(0.,0.,-1.), Vettore::new(0.,1.,0.), Vettore::new(1.,0.,0.), PI / 3.0);

    let scena = Scena::default();

    for x in 0..W{
        for y in 0..H {

            let mut rgb = Vettore::new(0.0,0.0,0.0);
            let mut rgb_iterante : Vettore;

            for _sample in 0..SAMPLES {
                
                camera.dir_pix = camera.genera_direzione(&(x as f64), &(y as f64), &(W as f64), &(H as f64));
                
                let info = test_collisione(&camera, &scena.oggetti); 
                
                if info.colpito == true {
                    rgb_iterante = Vettore::new(
                        127.0 * (info.norma_colpito.x + 1.0),
                        127.0 * (info.norma_colpito.y + 1.0),
                        127.0 * (info.norma_colpito.z + 1.0),
                    )

                } else {
                    rgb_iterante = Vettore::new(
                        camera.dir_pix.x.abs() * 255.0,
                        camera.dir_pix.y.abs() * 255.0,
                            255.0,
                    )
                }      
                
                rgb = rgb + rgb_iterante;

            }

            let rgb_mediato = rgb / SAMPLES as f64;
            let trapianto = rgb_mediato.clip(); 
            let trapianto_u8 = trapianto.to_u8();
            let index = ((W * y + x) * 3) as usize;
            pixels[index .. index + 3].copy_from_slice(&trapianto_u8);

        }
    }


    let _ = write_ppm("OUTPUT/output_rust.ppm", &pixels, W, H, 255);

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    let seconds = duration.as_secs_f64();

    println!("\n\nFinito in: {:.6} seconds\n\n", seconds);
    Ok(())

}
