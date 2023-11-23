
use std::fmt::Display;
use std::fmt::Formatter;
use crate::color::*;

pub struct ImageCoordinate {
    x: usize,
    y: usize,
}

pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    pixels: Vec<Vec<Pixel>>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    const PPM_X_RATIO: f64 = 256 as f64 / (WIDTH - 1) as f64;
    const PPM_Y_RATIO: f64 = 256 as f64 / (HEIGHT - 1) as f64;
    fn ppm_scale_x(x: usize) -> u8 {
        ((x as f64) * Self::PPM_X_RATIO) as u8
    }
    fn ppm_scale_y(y: usize) -> u8 {
        ((y as f64) * Self::PPM_Y_RATIO) as u8
    }

    pub fn generate_red_green_scan() -> Self {
        let mut pixels = vec![vec![Pixel::default(); WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                pixels[y][x] = Pixel::new(
                    Image::<WIDTH, HEIGHT>::ppm_scale_x(x),
                    Image::<WIDTH, HEIGHT>::ppm_scale_y(y),
                    0u8);
            }
        }

        Image { pixels }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for Image<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "P3 {} {} 255\n", WIDTH, HEIGHT).unwrap();
    
        for pixel in self.pixels.iter().flatten() {
            write!(f, "{}", pixel).unwrap();
        }

        Ok(())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Image<WIDTH, HEIGHT> {
    fn default() -> Self {
        // Initialize the pixels array with a default valuepub 
        Image {
            pixels: vec![vec![Pixel::default(); WIDTH]; HEIGHT],
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> std::ops::Index<ImageCoordinate> for Image<WIDTH, HEIGHT> {
    type Output = Pixel;
    fn index(&self, index: ImageCoordinate) -> &Self::Output {
        &self.pixels[index.y][index.x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> std::ops::IndexMut<ImageCoordinate> for Image<WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: ImageCoordinate) -> &mut Self::Output {
        &mut self.pixels[index.y][index.x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IntoIterator for Image<WIDTH, HEIGHT> {
    type Item = Pixel;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels
            .into_iter()
            .flatten()
            .collect::<Vec<Pixel>>()
            .into_iter()
    }
}
