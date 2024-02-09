pub mod file{

    use std::fs::File;
    use std::io::{self, Write};

    pub fn write_ppm(filename: &str, pixels: &[u8], image_w : i32, image_h : i32, max_value : i32) -> io::Result<()> {
        let mut file = File::create(filename)?;

        // Write the PPM header
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", image_w, image_h)?;
        writeln!(file, "{}", max_value)?;

        // Write the pixel data
        file.write_all(pixels)?;

        Ok(())
    }   
}