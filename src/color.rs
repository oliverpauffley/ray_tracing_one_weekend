use crate::vec::Color;

impl Color {
    pub fn write_color(&self) {
        print!("{}", &self);
    }
}
