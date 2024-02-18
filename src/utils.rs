pub mod file{

    use std::fmt;
    use std::io::{self, Write};
    use std::ops::{Add, Div, Mul, Sub, Neg};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crate::geometria::oggetti::{Materiale, Triangolo};

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
    
    pub fn read_lines_from_file(filename: &str) -> Result<Vec<String>, std::io::Error> {

        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut lines = Vec::new();
    
        for line in reader.lines() {
            lines.push(line?);
        }
    
        Ok(lines)
    }

    pub fn controllo_estrazione() -> Vec<Triangolo> {
        let mut triangoli : Vec<Triangolo> = vec![];
        match read_lines_from_file("modelli/m_ban.obj") {
            Ok(lines) => {
        
                let mut vertici : Vec<Vettore> = vec![];
                let mut links : Vec<Vettore> = vec![];

                for line in lines {
        
                    if let Some(primo_valore) = line.chars().next() {
                        
                        if primo_valore == 'v' {
        
                            // Split the string by whitespace and collect the parts into a vector
                            let parts: Vec<&str> = line.split_whitespace().collect();

                            // Parse the second, third, and fourth parts into f64 values
                            if let [_, x_str, y_str, z_str] = parts.as_slice() {
                                
                                if let (Ok(x), Ok(y), Ok(z)) = (x_str.parse::<f64>(), y_str.parse::<f64>(), z_str.parse::<f64>()) {
                                
                                    let vertice: Vettore = Vettore::new(x, y, z);
                                    vertici.push(vertice)
                                
                                } else {
                                
                                    println!("Error: Failed to parse coordinates");
                                
                                }
                            
                            } else {
                            
                                println!("Error: Unexpected number of parts");
                            
                            }
                            

        
                        } else if primo_valore == 'f' {
        
                            // Split the line by whitespace and skip the first element
                            let link: Vec<_> = line
                                .split_whitespace()
                                .skip(1)
                                // For each part, split by '//', take the first part, and parse it into an integer
                                .map(|x| x.split("//").next().unwrap().parse::<i32>().unwrap())
                                .collect();
                            
                            links.push(Vettore::new((link[0] as f64 - 1.0), (link[1] as f64 - 1.0), (link[2] as f64 - 1.0),))
        
                        }
                    }
                }

                let lunghezza_modello = links.len();

                for i in 0..lunghezza_modello {
                    triangoli.push(Triangolo::new(
                        Vettore::new(vertici[links[i].x as usize].x, vertici[links[i].x as usize].y, vertici[links[i].x as usize].z),
                        Vettore::new(vertici[links[i].y as usize].x, vertici[links[i].y as usize].y, vertici[links[i].y as usize].z),
                        Vettore::new(vertici[links[i].z as usize].x, vertici[links[i].z as usize].y, vertici[links[i].z as usize].z),
                        Materiale::new(Vettore::new(0.0, 0.0, 0.0), Vettore::new(i as f64 / lunghezza_modello as f64,0.0, 1.0 - i as f64 / lunghezza_modello as f64), 1.0, false, 0.0, 0.0, 0.0)
                    ))
                }

            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }

        triangoli

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

        pub fn cross(&self, v2 : &Vettore) -> Vettore {
            // Vettore::new(self.y * v2.z - self.z * v2.y, self.z * v2.x - self.x * v2.z, self.x * v2.y - self.y * v2.x)
            Vettore::new(v2.y * self.z - v2.z * self.y, v2.z * self.x - v2.x * self.z, v2.x * self.y - v2.y * self.x)
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

    // Implement the Display trait for the struct
    impl fmt::Display for Vettore {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Vec coord : {} {} {}", self.x, self.y, self.z)
        }
    }

}