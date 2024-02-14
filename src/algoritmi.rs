pub mod collisioni {

    use crate::geometria::oggetti::Sfera;
    use crate::camera::camera::Camera;
    use crate::Vettore;

    pub fn test_collisione(raggio : &Camera, oggetti : &[Sfera]) -> HitInfo {
        
        let mut risultato = HitInfo::new();
        let mut migliore_dx = 100000.0;

        for (index, oggetto) in oggetti.iter().enumerate() {
            
            let risultato_iter : f64;

            // per ogni tipo di oggetto esegue il suo test di collisione
            risultato_iter = oggetto.collisione_oggetto(raggio);
            
            if risultato_iter >= 0.0 {
                if risultato_iter < migliore_dx { 
                    migliore_dx = risultato_iter;
                    
                    risultato.colpito = true;
                    risultato.distanza = risultato_iter;
                    risultato.indice_sfera = index;
                    risultato.punto_colpito = oggetto.punto_colpito(risultato.distanza, raggio);
                    risultato.norma_colpito = oggetto.normale(risultato.punto_colpito);
                }
            }
            
        }
        risultato
    }

    pub struct HitInfo {
        pub colpito : bool,
        pub punto_colpito : Vettore,
        pub norma_colpito : Vettore,
        pub norma_rifrazione : Vettore,
        pub distanza : f64,
        pub indice_sfera : usize,
        pub front_face : bool
    }

    impl HitInfo {
        pub fn new() -> HitInfo {
            HitInfo {
                colpito : false,
                punto_colpito : Vettore::new(0.0, 0.0, 0.0),
                norma_colpito : Vettore::new(0.0, 0.0, 0.0),
                norma_rifrazione : Vettore::new(0.0, 0.0, 0.0),
                distanza : 0.0,
                indice_sfera : 0,
                front_face : true
            }
        }

        pub fn check_front_face(mut self, raggio : &Camera) -> HitInfo {
            if raggio.dir_pix.dot(&self.norma_colpito) > 0.0 {
                self.front_face = false;
                self.norma_rifrazione = - self.norma_colpito;
            } else {
                self.front_face = true;
                self.norma_rifrazione = self.norma_colpito;
            }

            self

        }

    }

}