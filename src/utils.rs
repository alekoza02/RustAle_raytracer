pub mod file{

    use std::fs::File;
    use std::io::{self, Write};
    use std::ops::{Add, Div, Mul, Sub, Neg};

    pub fn write_ppm(filename: &str, pixels: &[u8], image_w : i32, image_h : i32, max_value : i32) -> io::Result<()> {
        let mut file = File::create(filename)?;

        // PPM header
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", image_w, image_h)?;
        writeln!(file, "{}", max_value)?;

        // pixel data info
        file.write_all(pixels)?;

        Ok(())
    }

    // Derivazione dei tratti Clone e Copy
    #[derive(Clone, Copy)]
    pub struct Vettore {
        pub x : f64,
        pub y : f64,
        pub z : f64
    }

    impl Vettore {
        pub fn new(x : f64, y : f64, z : f64) -> Vettore {
            Vettore {
                x, y, z
            }
        }

        pub fn modulo(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }
    
        pub fn versore(&self) -> Vettore {
            let modulo : f64 = self.modulo();
            Vettore::new(self.x / modulo, self.y / modulo, self.z / modulo)
        }

        pub fn dot(&self, v2 : &Vettore) -> f64 {
            self.x * v2.x + self.y * v2.y + self.z * v2.z
        }

        pub fn clip(&self) -> Vettore {
            Vettore::new(
                if self.x >= 255.0 {255.0} else {self.x},
                if self.y >= 255.0 {255.0} else {self.y},
                if self.z >= 255.0 {255.0} else {self.z}
            )
        }

        // pub fn abs(&self) -> Vettore {
        //     Vettore::new(
        //         self.x.abs(),
        //         self.y.abs(),
        //         self.z.abs()
        //     )
        // }

        pub fn to_u8(&self) -> [u8; 3] {
            [
                self.x as u8,
                self.y as u8,
                self.z as u8,
            ]
        }

        pub fn lerp(&self, other : Vettore, percento : f64) -> Vettore {
            
            let mut risultato = Vettore::new(0.0, 0.0, 0.0);

            let delta_x = percento * (self.x - other.x);
            let delta_y = percento * (self.y - other.y);
            let delta_z = percento * (self.z - other.z); 
            
            risultato.x = other.x + delta_x;
            risultato.y = other.y + delta_y;
            risultato.z = other.z + delta_z;

            risultato
        }

        pub fn tone_mapping_base(mut self) -> Self {
            self.x = self.x.sqrt();
            self.y = self.y.sqrt();
            self.z = self.z.sqrt();
            self
        }

    }

    impl Add for Vettore {
        type Output = Vettore;

        fn add(self, other: Vettore) -> Vettore {
            Vettore {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl Add<f64> for Vettore {
        type Output = Vettore;

        fn add(self, value : f64) -> Vettore {
            Vettore {
                x: self.x + value,
                y: self.y + value,
                z: self.z + value,
            }
        }
    }
    
    impl Sub for Vettore {
        type Output = Vettore;

        fn sub(self, other: Vettore) -> Vettore {
            Vettore {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl Neg for Vettore {
        type Output = Vettore;

        fn neg(self) -> Vettore {
            Vettore {
                x: - self.x,
                y: - self.y,
                z: - self.z,
            }
        }
    }

    impl Mul<Vettore> for Vettore {
        type Output = Vettore;
    
        fn mul(self, other: Vettore) -> Vettore {
            Vettore {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z
            }
        }
    }
    
    impl Mul<f64> for Vettore {
        type Output = Vettore;
    
        fn mul(self, value: f64) -> Vettore {
            Vettore {
                x: self.x * value,
                y: self.y * value,
                z: self.z * value,
            }
        }
    }

    impl Div<f64> for Vettore {
        type Output = Vettore;
    
        fn div(self, value: f64) -> Vettore {
            Vettore {
                x: self.x / value,
                y: self.y / value,
                z: self.z / value,
            }
        }
    }

}