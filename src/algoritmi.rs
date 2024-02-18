pub mod collisioni {

    use crate::geometria::oggetti::{Sfera, Triangolo};
    use crate::camera::camera::Camera;
    use crate::Vettore;

    pub fn test_collisione(raggio : &Camera, oggetti_sfera : &[Sfera], oggetti_tri : &[Triangolo]) -> HitInfo {
        
        // inizializzazione record hit e distanza arbitraria come distanza di non collisione
        let mut risultato = HitInfo::new();
        let mut migliore_dx = 100000.0;

        // CICLO INTERSEZIONE SFERE
        for (index, oggetto) in oggetti_sfera.iter().enumerate() {
            
            let risultato_iter : f64;

            // per ogni tipo di oggetto esegue il suo test di collisione
            risultato_iter = oggetto.collisione_oggetto(raggio);
            
            // test se la distanza a cui abbiamo colpito un oggetto è positiva
            // tutti i test di collisione restituiscono -1.0 se non è avvenuta l'interazione
            if risultato_iter >= 0.0 {

                // test se l'oggetto colpito è il più vicino tra quelli analizzati fino ad ora
                if risultato_iter < migliore_dx { 
                    migliore_dx = risultato_iter;
                    
                    // popolamento record con l'oggetto colpito
                    risultato.colpito = true;
                    risultato.distanza = risultato_iter;
                    risultato.indice_oggetti_prox = index;
                    risultato.punto_colpito = oggetto.punto_colpito(risultato.distanza, raggio);
                    risultato.norma_colpito = oggetto.normale(risultato.punto_colpito);
                    risultato.tipo_oggetto = "sfera".to_string();
                }
            }
            
        }

        
        // CICLO INTERSEZIONE TRIANGOLI
        for (index, oggetto) in oggetti_tri.iter().enumerate() {
            
            let risultato_iter : f64;
            
            // per ogni tipo di oggetto esegue il suo test di collisione
            risultato_iter = oggetto.collisione_oggetto(raggio);
            
            // test se la distanza a cui abbiamo colpito un oggetto è positiva
            // tutti i test di collisione restituiscono -1.0 se non è avvenuta l'interazione
            if risultato_iter >= 0.0 {
                
                // test se l'oggetto colpito è il più vicino tra quelli analizzati fino ad ora
                if risultato_iter < migliore_dx { 
                    migliore_dx = risultato_iter;
                    
                    // popolamento record con l'oggetto colpito
                    risultato.colpito = true;
                    risultato.distanza = risultato_iter;
                    risultato.indice_oggetti_prox = index;
                    risultato.punto_colpito = oggetto.punto_colpito(risultato.distanza, raggio);
                    risultato.norma_colpito = oggetto.normale.versore();
                    risultato.tipo_oggetto = "triangolo".to_string();
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
        pub indice_oggetti_prox : usize,
        pub front_face : bool,
        pub tipo_oggetto : String
    }

    impl HitInfo {
        pub fn new() -> HitInfo {
            HitInfo {
                colpito : false,
                punto_colpito : Vettore::new(0.0, 0.0, 0.0),
                norma_colpito : Vettore::new(0.0, 0.0, 0.0),
                norma_rifrazione : Vettore::new(0.0, 0.0, 0.0),
                distanza : 0.0,
                indice_oggetti_prox : 0,
                front_face : true,
                tipo_oggetto : "None".to_string()
            }
        }

        pub fn check_front_face(mut self, raggio : &Camera) -> HitInfo {

            // controllo della faccia colpita basata sul prodotto scalare tra direzione e normale (puntante verso l'esterno)
            // norma rifrazione sarà la normale usata nei calcoli della rifrazione interna in cui il raggio parte dall'interno dell'oggetto
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