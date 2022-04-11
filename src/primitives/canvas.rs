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
}

#[cfg(test)]
#[path="tests/canvas_tests.rs"]
mod tests;