pub(crate) struct ColorOption(String, String);


trait Color {
    fn get_rgb(&mut self, r: i32, g: i32, b: i32);
    fn get_hex(&mut self, r: i32, g: i32, b: i32);
}

impl Color for ColorOption {

    fn get_rgb(&mut self, r:i32, g: i32, b: i32)  {
        self.0 = format!("{} {} {}", r, g, b);
    }

    fn get_hex(&mut self, r:i32, g: i32, b: i32) {
        self.1 = format!("{:x} {:x} {:x}", r, g, b);
    }

}

fn color_lizzard(r: i32, g: i32, b: i32) {

    let mut is_hooked: bool = false;

    while !is_hooked {

        if is_hooked == false {
            let _ = format!("{:x} {:x} {:x}", r, g, b);
        }

        is_hooked = true;
    }

}
