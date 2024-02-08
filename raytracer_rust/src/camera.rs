pub mod camera {
    use crate::matematica::operazioni_vettori::{dot_scalare, somma_wise, versore};
    use rand::prelude::*;

    pub struct Camera {
        pub pos : [f64; 3],
        pub dir : [f64; 3],
        pub ups : [f64; 3],
        pub rig : [f64; 3],
        pub fov : f64
    }

    impl Camera {
        pub fn new(pos : [f64; 3], dir : [f64; 3], ups : [f64; 3], rig : [f64; 3], fov : f64) -> Camera {
            Camera {
                pos,
                dir,
                ups,
                rig,
                fov
            }
        }

        pub fn genera_direzione(&self, x : &f64, y : &f64, w : &f64, h : &f64) -> [f64; 3] {

            let mut rng = rand::thread_rng();
            let random_x_offset = rng.gen_range(-1.0..1.0);
            let random_y_offset = rng.gen_range(-1.0..1.0);

            let ndc_x = (2.0 * (x + random_x_offset * 0.5 - 0.5) - w) / w;
            let ndc_y = (h - 2.0 * (y + random_y_offset * 0.5 - 0.5)) / h;

            let screen_x = ndc_x * (self.fov).tan() / 2.0;
            let screen_y = ndc_y * (self.fov).tan() / (2.0 * w/h);

            let raggio_direzione = somma_wise(&somma_wise(&self.dir, &dot_scalare(&self.rig, &screen_x)), &dot_scalare(&self.ups, &screen_y));
            let raggio_direzione_norm: [f64; 3] = versore(&raggio_direzione);

            raggio_direzione_norm
        }
    }
}