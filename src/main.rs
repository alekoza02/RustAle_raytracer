use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::f64::consts::PI;
use rand::prelude::*;
// use rand_distr::{Distribution, StandardNormal};
use std::time::Instant;

mod camera;
mod utils;
mod geometria;
mod algoritmi;

use algoritmi::collisioni::test_collisione;
use camera::camera::Camera;
use geometria::oggetti::Scena;
use utils::file::{write_ppm, Vettore};

// setting impostazioni
const W: usize = 720;
const H: usize = 720;
const SAMPLES: i32 = 4096;
const BOUNCES: i32 = 20;
const ZONE_COUNT: usize = 12;

fn render_zone(start_x: usize, end_x: usize, start_y: usize, end_y: usize, indice: i32, output: Arc<Mutex<Vec<u8>>>) {
    
    // inizializzazione utilities -> numeri random, camera, scena da renderizzare
    let mut rng = rand::thread_rng();
    
    // distribuzione normale
    // let std_nrm = StandardNormal;
    // let _numero_random_normale : f64 = std_nrm.sample(&mut rng);

    let mut camera = Camera::new(Vettore::new(0., 0., 30.), Vettore::new(0., 0., -1.), Vettore::new(0., 1., 0.), Vettore::new(1., 0., 0.), PI / 8.0);
    let scena = Scena::cornell_box_gloss();
    
    // metodo per tener traccia del progresso e aggiornare l'output
    let mut previous_progress = 0;

    for x in start_x..end_x {
        
        // aggiornamento output
        let progress : i32 = (100*(x-start_x)/(end_x-start_x)) as i32;

        if progress > previous_progress && progress % 5 == 0 && indice == 0{
            if indice == 0 {
                println!("Progresso : {}%", {progress})
            }
            previous_progress = progress;
            save_output_to_file(&output.lock().unwrap());
        }

        for y in start_y..end_y {
            
            // inizializzo colore del pixel (x, y)
            let mut rgb = Vettore::new(0.0, 0.0, 0.0);
            
            for _sample in 0..SAMPLES {
                
                // genero raggio con direzione (antialiasing : ON) diretta verso quel punto dello schermo con un certo POV 
                camera.dir_pix = camera.genera_direzione(&(x as f64), &(y as f64), &(W as f64), &(H as f64));
                camera.pos_iter = camera.pos;
                
                // inizializzo le informazioni sui raggi e colori
                let mut ray_incoming_light = Vettore::new(0., 0., 0.);
                let mut ray_color = Vettore::new(1., 1., 1.);
                let mut luce_emessa: Vettore;
                
                for _bounce in 0..BOUNCES {
                    
                    // eseguo il test di collisione con tutti gli oggetti della scena
                    // in futuro qui sarà presente prima il test BVH 
                    let info = test_collisione(&camera, &scena.oggetti);
                    
                    // se colpito calcola il colore dell'iterazione, altrimenti interrompe il ciclo
                    if info.colpito {
                        
                        // set nuova posizione di partenza per la prossima iterazione
                        camera.pos_iter = info.punto_colpito;

                        // alias per il materiale analizzato
                        let materiale_iterazione = &scena.oggetti[info.indice_sfera].materiale;
                        
                        // BLOCCO MATERIALE DIFFUSE / SPECULAR / GLOSS
                        if materiale_iterazione.vetro == false {
                        
                            // calcolo della direzione riflessa
                            let specular_ray = camera.dir_pix - info.norma_colpito * camera.dir_pix.dot(&info.norma_colpito) * 2.0;
                            
                            // calcolo della direzione random diffusa
                            let mut diffuse_ray = Vettore::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
                            diffuse_ray = diffuse_ray.versore();

                            // test del verso della direzione (se punta verso l'oggetto viene invertito)
                            if diffuse_ray.dot(&info.norma_colpito) < 0.0 {
                                diffuse_ray = -diffuse_ray;
                            }

                            // determino se l'iterazione considererà l'effetto glossiness o no -> true = 1, false = 0
                            let is_specular = materiale_iterazione.glossy >= rng.gen_range(0.0..1.0);
                            
                            // combinazione diffusa / speculare / glossy
                            camera.dir_pix = specular_ray.lerp(diffuse_ray, materiale_iterazione.roughness * is_specular as i32 as f64);
                            
                            // aggionramento colore e luce in base al contributo di questa iterazione
                            luce_emessa = materiale_iterazione.colore_emi * materiale_iterazione.forza_emi;
                            ray_incoming_light = ray_incoming_light + luce_emessa * ray_color;
                            ray_color = ray_color * Vettore::new(1.0, 1.0, 1.0).lerp(materiale_iterazione.colore, is_specular as i32 as f64);
                            

                        // BLOCCO MATERIALE VETRO
                        } else if materiale_iterazione.vetro {
                            
                            // calcolo della direzione rifratta:
                            // viene eseguito il check per raggio entrante ed uscente dall'oggetto. 
                            // nel caso di raggio uscente viene testato inoltre l'angolo limite.
                            // alla fine viene calcolata la probabilità di riflettanza dovuta alla legge di Brew.

                            // controllo raggio entrante / uscente basato sulla faccia colpita (interna / esterna)
                            let info = info.check_front_face(&camera);
                            
                            // calcolo del rapporto degli indici di rifrazione dei mezzi in base al raggio entrante / uscente (1.0 = aria)
                            let ratio_rifrazione = if info.front_face { 1.0 / materiale_iterazione.ir } else { materiale_iterazione.ir };
                            
                            // calcolo componenti trigonometriche
                            let coseno = (-camera.dir_pix).dot(&info.norma_rifrazione);
                            let seno = (1.0 - coseno.powi(2)).sqrt();
                            
                            // calcolo probabilità di riflettanza di Brew usando approx. di Schlick
                            let mut schlick_approx = (1.0 - materiale_iterazione.ir) / (1.0 + materiale_iterazione.ir);
                            schlick_approx = schlick_approx * schlick_approx;
                            
                            // condizioni di rifrazione : 1 = riflettanza, 2 = angolo limite
                            let cannot_refract1 = schlick_approx + (1.0 - schlick_approx) * ((1.0 - coseno).powi(5)) > rng.gen_range(0.0..1.0);
                            let cannot_refract2 = ratio_rifrazione * seno > 1.0;
                            
                            if cannot_refract2 | cannot_refract1 {
                                
                                // non può rifrarre -> riflessione basata sulla normale interna
                                camera.dir_pix = camera.dir_pix - info.norma_rifrazione * camera.dir_pix.dot(&info.norma_rifrazione) * 2.0;
                            
                            } else {
                            
                                // può rifrarre -> calcolo della nuova direzione con componente parallela e perpendicolare alla normale
                                let r_out_perp = (camera.dir_pix + info.norma_rifrazione * coseno) * ratio_rifrazione;
                                let r_out_para = -info.norma_rifrazione * (1.0 - r_out_perp.modulo().powi(2)).abs().sqrt();
                                camera.dir_pix = r_out_perp + r_out_para;
                            }

                            // aggionramento colore e luce in base al contributo di questa iterazione
                            luce_emessa = materiale_iterazione.colore_emi * materiale_iterazione.forza_emi;
                            ray_incoming_light = ray_incoming_light + luce_emessa * ray_color;
                            ray_color = ray_color * materiale_iterazione.colore;
                            
                        }
                    
                    } else {
                        break;
                    }
                }

                // sommatoria contributi samples
                rgb = rgb + ray_incoming_light * 255.0;
            }
            
            // media samples
            let rgb_mediato = rgb / SAMPLES as f64;
            
            // clip dei valori (aggiunta del tone-mapping)
            let trapianto = rgb_mediato.clip();

            // trasformazione in valori u8 (0-255) adatti per il formato .ppm
            let trapianto_u8 = trapianto.to_u8();

            // inserimento dei valori nel vettore condiviso multithread
            let index = ((W * y + x) * 3) as usize;
            let mut output = output.lock().unwrap();
            output[index..index + 3].copy_from_slice(&trapianto_u8);
        }
    }
    println!("\n-------------------\nPROCESSO {} FINITO\n-------------------", indice)
}

fn main() {
    
    // start timer
    let start_time = Instant::now();
    
    // La tipologia di output sarà un array condiviso di u8
    let output = Arc::new(Mutex::new(vec![0; W * H * 3]));
    
    // calcolo larghezza zone (4 * 3 = 12 thread totali)
    let zone_width = W / 4;
    let zone_height = H / 3;

    // lancio dei thread con i vari intervalli di renderizzazione
    (0..ZONE_COUNT).into_par_iter().for_each(|i| {
        
        let start_x = (i % 4) * zone_width;
        let end_x = start_x + zone_width;
        
        let start_y = (i / 4) * zone_height;
        let end_y = start_y + zone_height;
        
        render_zone(start_x, end_x, start_y, end_y, i as i32, Arc::clone(&output));
    });
    
    // salvataggio finale immagine
    save_output_to_file(&output.lock().unwrap());
    
    // print tempo impiegato
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    let seconds = duration.as_secs_f64();
    println!("\n\nFinito in: {:.3} seconds\n\n", seconds);

}

// salvataggio vettore di u8 in formato .ppm
fn save_output_to_file(output: &Vec<u8>) {
    let _ = write_ppm("OUTPUT/debug.ppm", &output, W as i32, H as i32, 255);
}
