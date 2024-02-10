mod camera;
mod utils;
mod geometria;
mod algoritmi;

use std::f64::consts::PI;

use rand::prelude::*;
use std::io::{self};
use std::time::Instant;

use algoritmi::collisioni::test_collisione;
use camera::camera::Camera;
use geometria::oggetti::Scena;
use utils::file::{write_ppm, Vettore};

const W : i32 = 1080;
const H : i32 = 1080;
const SAMPLES : i32 =  256;
const BOUNCES : i32 = 4;

fn main() -> io::Result<()> {

    let mut rng = rand::thread_rng();

    let start_time = Instant::now();
    let mut pixels: Vec<u8> = vec![0; (W * H * 3) as usize];
    // let mut pixels: [u8; (W*H*3) as usize] = [0; (W * H * 3) as usize];
    let mut camera = Camera::new(Vettore::new(0.,0.,30.), Vettore::new(0.,0.,-1.), Vettore::new(0.,1.,0.), Vettore::new(1.,0.,0.), PI / 8.0);

    let scena = Scena::cornell_box();

    for x in 0..W{
        for y in 0..H {

            let mut rgb = Vettore::new(0.0,0.0,0.0);

            for _sample in 0..SAMPLES {
                
                camera.dir_pix = camera.genera_direzione(&(x as f64), &(y as f64), &(W as f64), &(H as f64));
                
                let mut ray_incoming_light = Vettore::new(0.,0.,0.);
                let mut ray_color = Vettore::new(1.,1.,1.);
                let mut luce_emessa : Vettore;

                camera.pos_iter = camera.pos;
                
                for _bounce in 0..BOUNCES{
                    

                    let info = test_collisione(&camera, &scena.oggetti); 
                    
                    if info.colpito == true {
                        camera.pos_iter = info.punto_colpito;

                        camera.dir_pix = Vettore::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
                        camera.dir_pix = camera.dir_pix.versore();

                        if camera.dir_pix.dot(&info.norma_colpito) < 0.0 {
                            camera.dir_pix = - camera.dir_pix
                        }

                        luce_emessa = scena.oggetti[info.indice_sfera].materiale.colore_emi * scena.oggetti[info.indice_sfera].materiale.forza_emi;
                        ray_incoming_light = ray_incoming_light + luce_emessa * ray_color;
                        ray_color = ray_color * scena.oggetti[info.indice_sfera].materiale.colore;
                        
                    } else {
                        break
                    }      
                    
                }
            
                rgb = rgb + ray_incoming_light * 255.0;
                // println!("{},{},{}", rgb.x, rgb.y, rgb.z);
            
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
