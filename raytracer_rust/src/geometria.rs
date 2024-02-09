pub mod oggetti {

    use crate::camera::camera::Camera;
    use crate::Vettore;
    
    // possibili oggetti nella scena
    pub enum Oggetti {
        Sfera(Sfera),
    }

    // specifiche sulla sfera
    pub struct Sfera {
        origine : Vettore,
        raggio : f64
    }

    // gestione della scena totale
    pub struct Scena {
        pub oggetti : [Oggetti; 4],
        // luci : [Lucu]
    }

    // implementazione funzioni legate alla sfera
    impl Sfera {
        fn new(origine : Vettore, raggio : f64) -> Sfera {
            Sfera {
                origine,
                raggio
            }
        }

        pub fn normale(&self, punto_colpito : Vettore) -> Vettore {
            let normale = punto_colpito - self.origine;
            let risultato = normale.versore();
            risultato
        }

        pub fn punto_colpito(&self, distanza : f64, raggio : &Camera) -> Vettore {
            let risultato_1 = raggio.dir_pix * distanza;
            let risultato_2 = risultato_1 + raggio.pos;
            risultato_2
        }

        pub fn collisione_oggetto(&self, raggio : &Camera) -> f64 {
            
            let oc = raggio.pos - self.origine;
            let a : f64 = 1.0;
            let b : f64 = oc.dot(&raggio.dir_pix) * 2.0;
            let c : f64 = oc.dot(&oc) - self.raggio.powi(2);
            
            let discriminante = b.powi(2) - 4.0 * a * c;

            if discriminante >= 0.0 {
                return (- b - discriminante.sqrt()) / (2.0*a)
            } else {
                return -1.0
            }
        }
    }

    // implementazione scena default
    impl Scena {
        pub fn default() -> Scena {
            let argomento = [
                Oggetti::Sfera(Sfera::new(Vettore::new(0.,0.,0.), 10.)),
                Oggetti::Sfera(Sfera::new(Vettore::new(0.,-10010.,0.), 10000.)),
                Oggetti::Sfera(Sfera::new(Vettore::new(3.,3.,15.), 2.)),
                Oggetti::Sfera(Sfera::new(Vettore::new(-8.,-8.,-20.), 10.))
            ];

            Scena{oggetti : argomento}
        }
    }
}