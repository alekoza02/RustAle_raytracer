pub mod camera {
    use crate::Vettore;
    use rand::prelude::*;


    pub struct Camera {
        pub pos : Vettore,
        pub pos_iter : Vettore,
        pub dir : Vettore,
        pub dir_pix : Vettore,
        pub ups : Vettore,
        pub rig : Vettore,
        pub fov : f64
    }

    impl Camera {
        pub fn new(pos : Vettore, dir : Vettore, ups : Vettore, rig : Vettore, fov : f64) -> Camera {
            Camera {
                pos,
                pos_iter : pos,
                dir,
                dir_pix : Vettore::new(0.0,0.0,0.0),
                ups,
                rig,
                fov
            }
        }

        pub fn genera_direzione(&self, x : &f64, y : &f64, w : &f64, h : &f64) -> Vettore {

            // BLOCCO ANTIALIASING
            let mut rng = rand::thread_rng();
            let random_x_offset = rng.gen_range(-1.0..1.0);
            let random_y_offset = rng.gen_range(-1.0..1.0);

            let ndc_x = (2.0 * (x + random_x_offset) - w) / w;
            let ndc_y = (h - 2.0 * (y + random_y_offset)) / h;
            // BLOCCO ANTIALIASING

            // calcolo componente direzione basata sul FOV
            let screen_x = ndc_x * (self.fov).tan() / 2.0;
            let screen_y = ndc_y * (self.fov).tan() / (2.0 * w/h);

            let raggio_direzione : Vettore = self.dir + self.rig * screen_x + self.ups * screen_y;
            let raggio_direzione_norm : Vettore = raggio_direzione.versore();

            raggio_direzione_norm
        }

        pub fn depth_of_field_setup(mut self, x : &f64, y : &f64, w : &f64, h : &f64, focal_distance : f64) -> Self {

            // BLOCCO DEFOCUS
            let mut rng = rand::thread_rng();
            let random_x_offset = rng.gen_range(0.0..1.0);
            let random_y_offset = rng.gen_range(0.0..1.0);
            // BLOCCO DEFOCUS

            // calcolo nuova origine
            self.pos_iter = self.pos + self.rig * random_x_offset + self.ups * random_y_offset;
            
            // calcolo nuova origine (NO antialiasing)
            let ndc_x = (2.0 * x - w) / w;
            let ndc_y = (h - 2.0 * y) / h;
            // BLOCCO ANTIALIASING

            // calcolo componente direzione basata sul FOV
            let screen_x = ndc_x * (self.fov).tan() / 2.0;
            let screen_y = ndc_y * (self.fov).tan() / (2.0 * w/h);

            let raggio_direzione : Vettore = self.dir + self.rig * screen_x + self.ups * screen_y;
            let raggio_direzione_norm : Vettore = raggio_direzione.versore();

            // trovo un punto sul piano di focus e calcolo la nuova direzione
            let posizione_sul_piano_focus : Vettore = raggio_direzione_norm * focal_distance * raggio_direzione_norm.dot(&self.dir);
            self.dir_pix = posizione_sul_piano_focus - self.pos_iter + self.pos;
            self.dir_pix = self.dir_pix.versore();

            self

        }

    }
}