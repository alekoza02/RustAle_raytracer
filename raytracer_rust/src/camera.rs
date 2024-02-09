pub mod camera {
    use crate::Vettore;
    use rand::prelude::*;


    pub struct Camera {
        pub pos : Vettore,
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
                dir,
                dir_pix : Vettore::new(0.0,0.0,0.0),
                ups,
                rig,
                fov
            }
        }

        pub fn genera_direzione(&self, x : &f64, y : &f64, w : &f64, h : &f64) -> Vettore {

            let mut rng = rand::thread_rng();
            let random_x_offset = rng.gen_range(-0.1..0.1);
            let random_y_offset = rng.gen_range(-0.1..0.1);

            let ndc_x = (2.0 * (x + random_x_offset * 0.5 - 0.5) - w) / w;
            let ndc_y = (h - 2.0 * (y + random_y_offset * 0.5 - 0.5)) / h;

            let screen_x = ndc_x * (self.fov).tan() / 2.0;
            let screen_y = ndc_y * (self.fov).tan() / (2.0 * w/h);

            let raggio_direzione : Vettore = self.dir + self.rig * screen_x + self.ups * screen_y;
            let raggio_direzione_norm : Vettore = raggio_direzione.versore();

            raggio_direzione_norm
        }
    }
}