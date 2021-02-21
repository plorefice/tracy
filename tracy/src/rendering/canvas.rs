//! Virtual canvas to which the final image will be rendered.

use std::slice;

use super::Color;

/// A canvas is a rectangular grid of pixels, each with its own [`Color`].
#[derive(Debug, Default, Clone)]
pub struct Canvas {
    grid: Vec<Color>,
    width: u32,
    height: u32,
}

impl Canvas {
    /// Creates a new canvas with the specified size and all pixels initialized to black.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            grid: vec![Default::default(); (width * height) as usize],
            width,
            height,
        }
    }

    /// Returns the width of the canvas.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the canvas.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns an iterator over the pixels of this canvas.
    ///
    /// The canvas is traversed top-to-bottom, left-to-right.
    pub fn iter(&self) -> slice::Iter<Color> {
        self.grid.iter()
    }

    /// Returns a mutable iterator over the pixels of this canvas.
    ///
    /// The canvas is traversed top-to-bottom, left-to-right.
    pub fn iter_mut(&mut self) -> slice::IterMut<Color> {
        self.grid.iter_mut()
    }

    /// Return the `i`-th scanline of this canvas.
    pub fn scanline(&self, i: u32) -> Option<&[Color]> {
        self.grid
            .get((i * self.width) as usize..((i + 1) * self.width) as usize)
    }

    /// Sets the pixel at position `(x,y)` to the specified color.
    ///
    /// # Panics
    ///
    /// Panics if the specified position does not lie within the canvas.
    pub fn put(&mut self, x: u32, y: u32, c: Color) {
        if x < self.width() && y < self.height() {
            self.grid[(y * self.width + x) as usize] = c;
        }
    }

    /// Returns the color of the pixel at position `(x,y)`, or `None` is the position is not within
    /// the canvas.
    pub fn get(&self, x: u32, y: u32) -> Option<&Color> {
        self.grid.get((y * self.width + x) as usize)
    }

    /// Converts the canvas' contents to PPM format.
    pub fn convert_to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n{}\n", self.width(), self.height(), 255);
        ppm.reserve((self.width() * self.height() * 12) as usize);

        for y in 0..self.height() {
            let mut line_len = 0;

            for x in 0..self.width() {
                let (r, g, b) = self.get(x, y).unwrap().to_rgb888();

                // Lines should not be longer than 70 characters in PPM files.
                // Iterate over each color component in order to split lines as close as possible
                // to the 70 character mark.
                for val in &[r, g, b] {
                    let s = format!("{} ", val);

                    // Swap out the last space for a newline and reset the length counter
                    if line_len + s.len() > 70 {
                        ppm.pop();
                        ppm.push('\n');
                        line_len = 0;
                    }

                    ppm += &s;
                    line_len += s.len();
                }
            }

            ppm.pop();
            ppm.push('\n');
        }

        ppm
    }
}
