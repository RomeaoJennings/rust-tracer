use std::fs::OpenOptions;
use std::io::{Result, Write};

use super::RgbColor;

pub struct Canvas {
    pixels: Vec<Vec<RgbColor>>,
}

impl Canvas {
    pub fn new(mut width: usize, mut height: usize, background_color: Option<RgbColor>) -> Self {
        width = width.max(1);
        height = height.max(1);
        let background_color = background_color.unwrap_or(RgbColor::new(0.0, 0.0, 0.0));
        let pixels = vec![vec![background_color; width]; height];
        Canvas { pixels }
    }

    pub fn get_pixel(&self, row: usize, col: usize) -> RgbColor {
        self.pixels[row][col].clone()
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, color: RgbColor) {
        self.pixels[row][col] = color;
    }

    pub fn get_width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.pixels.len()
    }

    pub fn to_ppm(&self, filename: &str) -> Result<()> {
        let contents = self.build_contents();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(filename)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn build_contents(&self) -> String {
        let mut content = "P3\n".to_string();
        content.push_str(&self.pixels[0].len().to_string());
        content.push(' ');
        content.push_str(&self.pixels.len().to_string());
        content.push_str("\n255\n");
        for row in self.pixels.iter() {
            let mut first = true;
            for col in row.iter() {
                let (r, g, b) = col.to_u8_tuple();
                if !first {
                    content.push(' ');
                }
                first = false;
                content.push_str(&r.to_string());
                content.push(' ');
                content.push_str(&g.to_string());
                content.push(' ');
                content.push_str(&b.to_string());
            }
            content.push('\n');
        }
        content
    }
}

#[cfg(test)]
#[path = "tests/canvas_tests.rs"]
mod tests;
