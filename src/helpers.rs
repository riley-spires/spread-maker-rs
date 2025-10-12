use raylib::prelude::*;
use std::fs::File;
use std::io::prelude::Read;
use std::io::Error;

pub fn norm_to_reg(col: [f32; 3]) -> Color {
    Color{r: (col[0] * 255.0) as u8, g: (col[1] * 255.0) as u8, b: (col[2] * 255.0) as u8, a: 255}
}

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
