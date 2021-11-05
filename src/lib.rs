use image::{ImageBuffer, Rgb};
use imageproc::{
    drawing::{draw_antialiased_line_segment_mut, draw_filled_circle_mut},
    pixelops::interpolate,
};
use palette::{convert::IntoColor, Hsl, Pixel, Srgb};
use std::f64::consts::PI;

const IMG_SIZE: u32 = 256;
const IMG_CENTER: (i32, i32) = (IMG_SIZE as i32 / 2, IMG_SIZE as i32 / 2);
const RADIUS: f64 = 96.0;
const RADIUS_SWAY: f64 = 16.0;
const REPETITIONS: usize = 3;

pub struct Img {
    points: [u8; 64],
}

impl Img {
    pub fn new(seed: [u8; 64]) -> Img {
        Img { points: seed }
    }

    pub fn render(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(IMG_SIZE, IMG_SIZE);

        let hue = (self.points[0] as u16 ^ (self.points[1] as u16) << 8) as f32 / 65535. * 360.;
        let saturation = (self.points[2] as u16 ^ (self.points[3] as u16) << 8) as f32 / 65535.;

        let fg_color = get_color(hue, saturation, 1.0);

        // Draw circles
        for i in 0..16 {
            let bg_color = get_color(
                hue + self.points[16 + i] as f32 / 255.0 * 30.,
                saturation,
                (self.points[32 + i] as u16 ^ (self.points[48 + i] as u16) << 8) as f32
                    / 65535.0
                    / 4.0,
            );

            draw_filled_circle_mut(
                &mut img,
                IMG_CENTER,
                IMG_SIZE as i32 * (16 - i as i32) / 16,
                bg_color,
            );
        }

        // Draw wave
        let positions = (0..self.points.len() * REPETITIONS)
            .map(|i| {
                let angle = i as f64 * 2.0 * PI / (self.points.len() * REPETITIONS) as f64;
                let length = (self.points[i as usize % self.points.len()] as f64 - 128.0) / 128.0
                    * RADIUS_SWAY
                    + RADIUS;
                ptoc(angle, length)
            })
            .collect::<Vec<_>>();

        for i in 0..positions.len() {
            let pos = positions[i];
            let npos = positions[(i + 1) % positions.len()];

            draw_antialiased_line_segment_mut(&mut img, pos, npos, fg_color, interpolate);
        }

        img
    }
}

/// Convert polar to cartesian coordinates
fn ptoc(angle: f64, length: f64) -> (i32, i32) {
    let x = length * angle.cos();
    let y = length * angle.sin();

    (x as i32 + IMG_CENTER.0, y as i32 + IMG_CENTER.1)
}

/// Get an Rgb<u8> color from HSL values
fn get_color(hue: f32, saturation: f32, lightness: f32) -> Rgb<u8> {
    let col: Srgb = Hsl::new(hue, saturation, lightness).into_color();
    let col: [f32; 3] = col.into_raw();

    Rgb {
        0: [
            (col[0] * 256.) as u8,
            (col[1] * 256.) as u8,
            (col[2] * 256.) as u8,
        ],
    }
}
