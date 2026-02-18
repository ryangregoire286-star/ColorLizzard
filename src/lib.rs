
#[warn(dead_code)]
struct ColorOption(String, String);

impl ColorOption {

    fn get_rgb(&mut self, r:i32, g: i32, b: i32)  {
        self.0 = format!("{} {} {}", r, g, b);
    }

    fn get_hex(&mut self, r:i32, g: i32, b: i32) {
        self.1 = format!("{:x} {:x} {:x}", r, g, b);
    }
    cargo audit

}

fn color_lizzard(r: i32, g: i32, b: i32) {
    let _ = format!("{:x} {:x} {:x}", r, g, b);
}
