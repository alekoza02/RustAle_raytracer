use rayon::prelude::*;
use std::sync::{Arc, Mutex};

mod camera;
mod utils;
mod geometria;
mod algoritmi;

use std::f64::consts::PI;

use rand::prelude::*;
use std::time::Instant;

use algoritmi::collisioni::test_collisione;
use camera::camera::Camera;
use geometria::oggetti::Scena;
use utils::file::{write_ppm, Vettore};

const W : usize = 1800 / 4;
const H : usize = 1800 / 4;
const SAMPLES : i32 = 32;
const BOUNCES : i32 = 20;
const ZONE_COUNT: usize = 12;

fn render_zone(start_x: usize, end_x: usize, start_y: usize, end_y: usize, indice : u64, output: Arc<Mutex<Vec<u8>>>) {
    
    let mut rng = rand::thread_rng();

    // let mut pixels: [u8; (W*H*3) as usize] = [0; (W * H * 3) as usize];
    let mut camera = Camera::new(Vettore::new(0.,0.,30.), Vettore::new(0.,0.,-1.), Vettore::new(0.,1.,0.), Vettore::new(1.,0.,0.), PI / 8.0);

    let scena = Scena::cornell_box();

    for x in start_x..end_x {
         
        for y in start_y..end_y {

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

                        let materiale_iterazione = &scena.oggetti[info.indice_sfera].materiale;

                        
                        if materiale_iterazione.diffuse == true {
                            camera.dir_pix = Vettore::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
                            camera.dir_pix = camera.dir_pix.versore();
                            
                            if camera.dir_pix.dot(&info.norma_colpito) < 0.0 {
                                camera.dir_pix = - camera.dir_pix
                            }
                        
                        } else if materiale_iterazione.metallo == true {
                            camera.dir_pix = camera.dir_pix - info.norma_colpito * camera.dir_pix.dot(&info.norma_colpito) * 2.0;
                            camera.dir_pix = camera.dir_pix.versore();
                        
                        } else if materiale_iterazione.vetro == true {
                            
                            let info = info.check_front_face(&camera);

                            let ratio_rifrazione = if info.front_face == true {1.0 / materiale_iterazione.ir} else {materiale_iterazione.ir};
                            
                            let coseno = (-camera.dir_pix).dot(&info.norma_rifrazione);
                            let seno = (1.0 - coseno.powi(2)).sqrt();

                            let mut schlick_approx = (1.0-materiale_iterazione.ir) / (1.0+materiale_iterazione.ir);
                            schlick_approx = schlick_approx * schlick_approx;
                            let cannot_refract1 = schlick_approx + (1.0-schlick_approx)*((1.0 - coseno).powi(5)) > rng.gen_range(0.0..1.0);

                            let cannot_refract2 = ratio_rifrazione * seno > 1.0;
                            
                            if cannot_refract2 | cannot_refract1 == true{
                                camera.dir_pix = camera.dir_pix - info.norma_rifrazione * camera.dir_pix.dot(&info.norma_rifrazione) * 2.0;
                                camera.dir_pix = camera.dir_pix.versore();
                            } else {
                                
                                let r_out_perp = (camera.dir_pix + info.norma_rifrazione * coseno) * ratio_rifrazione;
                                let r_out_para = - info.norma_rifrazione * (1.0 - r_out_perp.modulo().powi(2)).abs().sqrt();
                                camera.dir_pix = r_out_perp + r_out_para;

                                camera.dir_pix = camera.dir_pix.versore();
                            }
                        }
                        
                        luce_emessa = materiale_iterazione.colore_emi * materiale_iterazione.forza_emi;
                        ray_incoming_light = ray_incoming_light + luce_emessa * ray_color;
                        ray_color = ray_color * materiale_iterazione.colore;
                        
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
            let mut output = output.lock().unwrap();
            output[index .. index + 3].copy_from_slice(&trapianto_u8);
        }
    }
}

fn main() {
    
    let start_time = Instant::now();
    // Initialize the output vector
    let output = Arc::new(Mutex::new(vec![0; W * H * 3]));

    // Split the image into zones
    let zone_width = W / 4;
    let zone_height = H / 3;

    // Render zones in parallel
    (0..ZONE_COUNT).into_par_iter().for_each(|i| {
        let start_x = (i % 4) * zone_width;
        let end_x = start_x + zone_width;
        let start_y = (i / 4) * zone_height;
        let end_y = start_y + zone_height;
        render_zone(start_x, end_x, start_y, end_y, i as u64, Arc::clone(&output));
    });

    // Call a function to save the output to a file
    save_output_to_file(&output.lock().unwrap());
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    let seconds = duration.as_secs_f64();

    println!("\n\nFinito in: {:.6} seconds\n\n", seconds);
}

fn save_output_to_file(output: &Vec<u8>) {
    // Function to save the output vector to a file
    // Implement your file-saving logic here
    let _ = write_ppm("OUTPUT/rust_parallelo_tracer.ppm", &output, W as i32, H as i32, 255);
}
