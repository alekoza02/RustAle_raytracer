pub mod oggetti {

    use crate::camera::camera::Camera;
    use crate::Vettore;
    
    // specifiche sulla sfera
    pub struct Sfera {
        pub origine : Vettore,
        pub raggio : f64,
        pub materiale : Materiale,
    }

    // gestione della scena totale
    pub struct Scena {
        pub oggetti : Vec<Sfera>,
    }

    // implementazione funzioni legate alla sfera
    impl Sfera {
        fn new(origine : Vettore, raggio : f64, materiale : Materiale) -> Sfera {
            Sfera {
                origine,
                raggio,
                materiale
            }
        }

        pub fn normale(&self, punto_colpito : Vettore) -> Vettore {
            let normale = punto_colpito - self.origine;
            let risultato = normale.versore();
            risultato
        }

        pub fn punto_colpito(&self, distanza : f64, raggio : &Camera) -> Vettore {
            let risultato_1 = raggio.dir_pix * distanza;
            let risultato_2 = risultato_1 + raggio.pos_iter;
            risultato_2
        }

        pub fn collisione_oggetto(&self, raggio : &Camera) -> f64 {
            
            // BLOCCO COLLISIONE RAGGIO SFERA
            let oc = raggio.pos_iter - self.origine;
            let a : f64 = 1.0;
            let b : f64 = oc.dot(&raggio.dir_pix) * 2.0;
            let c : f64 = oc.dot(&oc) - self.raggio.powi(2);
            
            let discriminante = b.powi(2) - 4.0 * a * c;
            // BLOCCO COLLISIONE RAGGIO SFERA

            // test delle soluzioni dell'equazione, sarà la soluzione positiva minore
            if discriminante >= 0.0 {
                let delta_min = (- b - discriminante.sqrt()) / (2.0*a);
                let delta_max = (- b + discriminante.sqrt()) / (2.0*a);

                // il threshold è impostato su 0.001 per evitare che il raggio rifratto interpreti la sua origine come punto di collisione
                if delta_min > 0.001 {delta_min} else if delta_max > 0.001 {delta_max} else {-1.0}

            } else {
                -1.0
            }
        }
    }

    // implementazione scena default
    impl Scena {
        // pub fn default() -> Scena {
        //     let argomento = vec![
        //         Sfera::new(Vettore::new(0.,0.,0.), 10.,         Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., true, false, false, 1.0)),
        //         Sfera::new(Vettore::new(0.,-10010.,0.), 10000., Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., true, false, false, 1.0)),
        //         Sfera::new(Vettore::new(3.,3.,15.), 2.,         Materiale::new(Vettore::new(0.,0.,0.), Vettore::new(0.,0.,1.), 15., true, false, false, 1.0)),
        //         Sfera::new(Vettore::new(-8.,-8.,-20.), 10.,     Materiale::new(Vettore::new(0.,0.,0.), Vettore::new(0.,1.,0.), 4., true, false, false, 1.0))
        //     ];

        //     Scena{oggetti : argomento}
        // }
        
        pub fn cornell_box() -> Scena {
            let argomento = vec![
                Sfera::new(Vettore::new(0.,-1005.,0.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, false, 1.0, 1.0)),
                Sfera::new(Vettore::new(0.,1005.,0.),       1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, false, 1.0, 1.0)),
                Sfera::new(Vettore::new(1005.,0.,0.),       1000.,  Materiale::new(Vettore::new(0.,0.7,1.), Vettore::new(0.,0.,1.), 0., false, false, 1.0, 1.0)),
                Sfera::new(Vettore::new(-1005.,0.,0.),      1000.,  Materiale::new(Vettore::new(1.,0.5,0.), Vettore::new(0.,0.,0.), 0., false, false, 1.0, 1.0)),
                Sfera::new(Vettore::new(0.,0.,-1005.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, false, 1.0, 1.0)),
                Sfera::new(Vettore::new(0.,12.,0.),         8.,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(1.,1.,1.), 3., false, false, 1.0, 1.0)),
                Sfera::new(Vettore::new(1.,-2.,-2.),        3.,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., true, false, 1.0, 0.2)),
                Sfera::new(Vettore::new(2.5,-3.25,4.),      1.75,   Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, false, 1.0, 1.0)),
                Sfera::new(Vettore::new(-3., -3.75,2.),     1.25,   Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, true, 1.5, 0.1))
            ];

            Scena{oggetti : argomento}
        }

    }

    // materiali
    pub struct Materiale {
        pub colore : Vettore,
        pub colore_emi : Vettore,
        pub forza_emi : f64,
        pub metallo : bool,
        pub vetro : bool,
        pub ir : f64,
        pub roughness : f64
    }

    impl Materiale {
        pub fn new(colore : Vettore, colore_emi : Vettore, forza_emi : f64, metallo : bool, vetro : bool, ir : f64, roughness : f64) -> Materiale {
            Materiale {
                colore,
                colore_emi,
                forza_emi,
                metallo,
                vetro,
                ir,
                roughness
            }
        }
    }

}