mod structs;

pub trait Color {
    fn get_rgb(&mut self, r: i32, g: i32, b: i32);
    fn get_hex(&mut self, r: i32, g: i32, b: i32);
}

pub struct ColorLizzard {
    pub(crate) _0: String,
    pub(crate) _1: String,
}


impl Color for ColorLizzard {
    fn get_rgb(&mut self, r: i32, g: i32, b: i32) {
        self._0 = format!("{} {} {}", r, g, b);
    }

    fn get_hex(&mut self, r: i32, g: i32, b: i32) {
        self._1 = format!("{:x} {:x} {:x}", r, g, b);
    }
}