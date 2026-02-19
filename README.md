##
## Package Title 
### Color Lizzard

### This Library Basically Converts 2 Red, Green, Blue Color Values

----------------
### Install Command
    
    color_lizzard = "=0.1.3"
------------------------
### Add Command

    cargo add color_lizzard@=0.1.3
--------------------------

----------
### Repo Link

### https://crates.io/crates/color_lizzard/0.1.3


----------------------

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

---------------------

### Current Version is Displayed on Screen