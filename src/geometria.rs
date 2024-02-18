pub mod oggetti {

    use crate::camera::camera::Camera;
    use crate::Vettore;
    
    // specifiche sulla sfera
    pub struct Sfera {
        pub origine : Vettore,
        pub raggio : f64,
        pub materiale : Materiale,
    }

    // specifiche sul triangolo
    pub struct Triangolo {
        pub v0 : Vettore,
        pub v1 : Vettore,
        pub v2 : Vettore,
        pub normale : Vettore,
        pub materiale : Materiale,
    }

    // gestione della scena totale
    pub struct Scena {
        pub oggetti_sfere : Vec<Sfera>,
        pub oggetti_tri : Vec<Triangolo>,
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

    // implementazione funzioni legate alla sfera
    impl Triangolo {
        pub fn new(v0 : Vettore, v1 : Vettore, v2 : Vettore, materiale : Materiale) -> Triangolo {
            let mut risultato = Triangolo {
                v0,
                v1,
                v2,
                normale : Vettore::new(0.0, 0.0, 0.0),
                materiale
            };

            // precalcolo della normale
            risultato.normale = risultato.normale();
            risultato
        }

        pub fn normale(&self) -> Vettore {
            let ab = self.v1 - self.v0;
            let ac = self.v2 - self.v0;
            ab.cross(&ac)
        }

        pub fn punto_colpito(&self, distanza : f64, raggio : &Camera) -> Vettore {
            let risultato_1 = raggio.dir_pix * distanza;
            let risultato_2 = risultato_1 + raggio.pos_iter;
            risultato_2
        }

        pub fn collisione_oggetto(&self, raggio : &Camera) -> f64 {
            
            // BLOCCO COLLISIONE RAGGIO TRIANGOLO
            let ab = self.v1 - self.v0;
            let ac = self.v2 - self.v0;
            let ao = raggio.pos_iter - self.v0;
            let dao = ao.cross(&raggio.dir_pix);

            let determinante = - raggio.dir_pix.dot(&self.normale);
            let inv_determin = 1.0 / determinante;

            let dst = ao.dot(&self.normale) * inv_determin;
            let u = ac.dot(&dao) * inv_determin;
            let v = - ab.dot(&dao) * inv_determin;
            let w = 1.0 - u - v;
            // BLOCCO COLLISIONE RAGGIO TRIANGOLO

            // test delle soluzioni dell'equazione
            if determinante > 0.0 && dst > 0.001 && u >= 0.0 && v >= 0.0 && w >= 0.0 {
                dst
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

        //     Scena{oggetti_sfere : argomento, oggetti_tri : vec![]}
        // }
        
        pub fn cornell_box() -> Scena {
            let argomento = vec![
                Sfera::new(Vettore::new(0.,-1005.,0.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(0.,1005.,0.),       1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(1005.,0.,0.),       1000.,  Materiale::new(Vettore::new(0.,0.7,1.), Vettore::new(0.,0.,1.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(-1005.,0.,0.),      1000.,  Materiale::new(Vettore::new(1.,0.5,0.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(0.,0.,-1005.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(0.,12.,0.),         8.,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(1.,1.,1.), 3., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(1.,-2.,-2.),        3.,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 1.0, 1.0)),
                Sfera::new(Vettore::new(2.5,-3.25,4.),      1.75,   Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(-3., -3.75,2.),     1.25,   Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., true, 1.5, 0.1, 0.0))
            ];

            Scena{oggetti_sfere : argomento, oggetti_tri : vec![]}
        }

        pub fn cornell_box_triangolo() -> Scena {
            let argomento_sfere = vec![
                Sfera::new(Vettore::new(0.,-1005.,0.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(0.,1005.,0.),       1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(1005.,0.,0.),       1000.,  Materiale::new(Vettore::new(0.,0.7,1.), Vettore::new(0.,0.,1.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(-1005.,0.,0.),      1000.,  Materiale::new(Vettore::new(1.,0.5,0.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(0.,0.,-1005.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(0.,4.,-2.),        0.5,     Materiale::new(Vettore::new(0.,0.,0.), Vettore::new(1.,1.,1.), 10., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(3.5,-3.25,6.),      0.5,   Materiale::new(Vettore::new(0.,0.,0.), Vettore::new(1.,1.,1.), 10., false, 1.0, 0.0, 0.0)),
                Sfera::new(Vettore::new(-4., -3.75,2.),     0.5,   Materiale::new(Vettore::new(0.,0.,0.), Vettore::new(1.,1.,1.), 10., false, 1.0, 0.0, 0.0))
            ];

            let argomento_tri = vec![
                Triangolo::new(Vettore::new(0.,4.,-2.), Vettore::new(3.5,-3.25,6.), Vettore::new(-4., -3.75,2.), Materiale::new(Vettore::new(0.,0.,0.), Vettore::new(1.,1.,1.), 10., false, 1.0, 0.0, 0.0))
            ];

            Scena{oggetti_sfere : argomento_sfere, oggetti_tri : argomento_tri}
        }

        pub fn cornell_box_banshee() -> Scena {
            Scena{oggetti_sfere : vec![], oggetti_tri : vec![]}
        }

        pub fn cornell_box_gloss() -> Scena {
                let argomento = vec![
                    Sfera::new(Vettore::new(0.,-1005.,0.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                    Sfera::new(Vettore::new(0.,1005.,0.),       1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                    Sfera::new(Vettore::new(1005.,0.,0.),       1000.,  Materiale::new(Vettore::new(0.,0.7,1.), Vettore::new(0.,0.,1.), 0., false, 1.0, 0.0, 0.0)),
                    Sfera::new(Vettore::new(-1005.,0.,0.),      1000.,  Materiale::new(Vettore::new(1.,0.5,0.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                    Sfera::new(Vettore::new(0.,0.,-1005.),      1000.,  Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 0.0, 0.0)),
                    Sfera::new(Vettore::new(0.,12.,0.),         8.,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(1.,1.,1.), 3., false, 1.0, 0.0, 0.0)),
                    Sfera::new(Vettore::new(-3.75,-3.8,0.0),        1.2,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 1.0, 1.0)),
                    Sfera::new(Vettore::new(-1.25,-3.8,0.0),        1.2,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 1.0, 0.5)),
                    Sfera::new(Vettore::new(1.25,-3.8,0.0),        1.2,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 1.0, 0.1)),
                    Sfera::new(Vettore::new(3.75,-3.8,0.0),        1.2,     Materiale::new(Vettore::new(1.,1.,1.), Vettore::new(0.,0.,0.), 0., false, 1.0, 1.0, 0.0))
                ];
    
                Scena{oggetti_sfere : argomento, oggetti_tri : vec![]}
            }

    }

    // materiali
    // Derivazione dei tratti Clone e Copy
    #[derive(Clone, Copy)]
    pub struct Materiale {
        pub colore : Vettore,
        pub colore_emi : Vettore,
        pub forza_emi : f64,
        pub vetro : bool,
        pub ir : f64,
        pub roughness : f64,
        pub glossy : f64
    }

    impl Materiale {
        pub fn new(colore : Vettore, colore_emi : Vettore, forza_emi : f64, vetro : bool, ir : f64, roughness : f64, glossy : f64) -> Materiale {
            Materiale {
                colore,
                colore_emi,
                forza_emi,
                vetro,
                ir,
                roughness,
                glossy
            }
        }
    }

}