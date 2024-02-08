pub mod operazioni_vettori {

    pub fn modulo(v : &[f64; 3]) -> f64 {
        (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt()
    }

    pub fn versore(v : &[f64; 3]) -> [f64; 3] {
        let modulo : f64 = modulo(v);
        [v[0] / modulo, v[1] / modulo, v[2] / modulo]
    }

    pub fn somma_wise(v1 : &[f64; 3], v2 : &[f64; 3]) -> [f64; 3] {
        [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
    }

    pub fn dot_scalare(v1 : &[f64; 3], value : &f64) -> [f64; 3] {
        [v1[0] * value, v1[1] * value, v1[2] * value]
    }

}