pub mod oggetti {

    use crate::camera::camera::Camera;
    use crate::matematica::operazioni_vettori::{dot, dot_scalare, somma_wise, sub_wise, versore};

    // possibili oggetti nella scena
    pub enum Oggetti {
        Sfera(Sfera),
    }

    // specifiche sulla sfera
    pub struct Sfera {
        origine : [f64; 3],
        raggio : f64
    }

    // gestione della scena totale
    pub struct Scena {
        pub oggetti : [Oggetti; 4],
        // luci : [Lucu]
    }

    // implementazione funzioni legate alla sfera
    impl Sfera {
        fn new(origine : [f64; 3], raggio : f64) -> Sfera {
            Sfera {
                origine,
                raggio
            }
        }

        pub fn normale(&self, punto_colpito : &[f64; 3], raggio : &Camera) -> [f64; 3] {
            let normale = sub_wise(&punto_colpito, &self.origine);
            let risultato = versore(&normale);
            risultato
        }

        pub fn punto_colpito(&self, distanza : &f64, raggio : &Camera) -> [f64; 3] {
            let risultato_1 = dot_scalare(&raggio.dir_pix, &distanza);
            let risultato_2 = somma_wise(&risultato_1, &raggio.pos);
            risultato_2
        }

        pub fn collisione_oggetto(&self, raggio : &Camera) -> f64 {
            
            let oc : [f64; 3] = sub_wise(&raggio.pos, &self.origine);
            let a : f64 = dot(&raggio.dir_pix, &raggio.dir_pix);
            let b : f64 = dot(&oc, &raggio.dir_pix) * 2.0;
            let c : f64 = dot(&oc, &oc) - self.raggio.powi(2);
            
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
                Oggetti::Sfera(Sfera::new([0.,0.,0.], 10.)),
                Oggetti::Sfera(Sfera::new([0.,-10010.,0.], 10000.)),
                Oggetti::Sfera(Sfera::new([3.,3.,15.], 2.)),
                Oggetti::Sfera(Sfera::new([-8.,-8.,-20.], 10.))
            ];

            Scena{oggetti : argomento}
        }
    }
}