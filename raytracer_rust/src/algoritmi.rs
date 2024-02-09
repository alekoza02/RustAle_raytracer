pub mod collisioni {

    use crate::geometria::oggetti::{self, Oggetti};
    use crate::camera::camera::Camera;

    pub fn test_collisione(raggio : &Camera, oggetti : &[Oggetti]) -> HitInfo {
        
        let mut risultato = HitInfo::new();
        let mut migliore_dx = 100000.0;

        for (index, oggetto) in oggetti.iter().enumerate() {
            
            let risultato_iter : f64;

            // per ogni tipo di oggetto esegue il suo test di collisione
            match oggetto {
                Oggetti::Sfera(Sfera) => {
                    risultato_iter = Sfera.collisione_oggetto(raggio);
                    
                    if risultato_iter >= 0.0 {
                        if risultato_iter < migliore_dx { 
                            migliore_dx = risultato_iter;
                            
                            risultato.colpito = true;
                            risultato.distanza = risultato_iter;
                            risultato.indice_sfera = index;
                            risultato.punto_colpito = Sfera.punto_colpito(&risultato.distanza, &raggio);
                            risultato.norma_colpito = Sfera.normale(&risultato.punto_colpito, &raggio);
                        }
                    }
                }
            }
            
        }
    risultato
    }

    pub struct HitInfo {
        pub colpito : bool,
        pub punto_colpito : [f64; 3],
        pub norma_colpito : [f64; 3],
        pub distanza : f64,
        pub indice_sfera : usize
    }

    impl HitInfo {
        fn new() -> HitInfo {
            HitInfo {
                colpito : false,
                punto_colpito : [0.0, 0.0, 0.0],
                norma_colpito : [0.0, 0.0, 0.0],
                distanza : 0.0,
                indice_sfera : 0
            }
        }
    }

}