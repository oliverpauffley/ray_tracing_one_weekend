use crate::vec::Color;

impl Color {
    pub fn write_color(&self, samples_per_pixel: usize) {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (self.0 * scale).sqrt().clamp(0.0, 0.999);
        let g = (self.1 * scale).sqrt().clamp(0.0, 0.999);
        let b = (self.2 * scale).sqrt().clamp(0.0, 0.999);

        let ir = (256.0 * r) as i64;
        let ig = (256.0 * g) as i64;
        let ib = (256.0 * b) as i64;
        print!("{} {} {}\n", ir, ig, ib);
    }
}
