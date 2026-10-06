#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Argb {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Argb {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self {
            a: a,
            r: r,
            g: g,
            b: b,
        }
    }
    pub fn as_precalculated_alpha(&self) -> u32 {
        if self.a == 0 {
            return 0;
        }
        if self.a == 255 {
            return ((self.a as u32) << 24)
                | ((self.r as u32) << 16)
                | ((self.g as u32) << 8)
                | (self.b as u32);
        }
        ((self.a as u32) << 24)
            | (((self.r as u32 * self.a as u32 + 127_u32) / 255) << 16)
            | (((self.g as u32 * self.a as u32 + 127_u32) / 255) << 8)
            | ((self.b as u32 * self.a as u32 + 127_u32) / 255)
    }
    pub fn precalculate_alpha_mut(&mut self) {
        if self.a == 0 {
            self.a = 0;
            self.r = 0;
            self.g = 0;
            self.b = 0;
        } else if self.a == 255 {
        } else {
            self.r = ((self.r as u32 * self.a as u32 + 127_u32) / 255) as u8;
            self.g = ((self.g as u32 * self.a as u32 + 127_u32) / 255) as u8;
            self.b = ((self.b as u32 * self.a as u32 + 127_u32) / 255) as u8;
        }
    }
    // 100% safest functions ever
    pub fn rotate_right(&mut self, n: u32) {
        // These are safe becuase b can be cast to a u32 becuase the struct essentially contains a u32
        let r: *mut u8 = &mut self.b;
        let ptr = r as *mut u32;
        unsafe {
            *ptr = (self.b as u32).rotate_right(n);
        }
    }
    pub fn rotate_left(&mut self, n: u32) {
        let r: *mut u8 = &mut self.b;
        let ptr = r as *mut u32;
        unsafe {
            *ptr = (self.b as u32).rotate_left(n);
        }
    }
    pub fn swap_r_b(&mut self) {
        self.r ^= self.b;
        self.b ^= self.r;
        self.r ^= self.b;
    }
}

#[test]
fn full_alpha_test() {
    let pixel = Argb::new(255, 1, 2, 3);
    assert_eq!(pixel.as_precalculated_alpha(), 4278256131_u32)
}

#[test]
fn partial_alpha_test() {
    let pixel = Argb::new(128, 1, 1, 1);
    assert_eq!(pixel.as_precalculated_alpha(), 2147549441_u32)
}

// This test is preety useless
#[test]
fn partialeq_default_test() {
    let r1 = Argb::default();
    let r2 = Argb {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    assert_eq!(r1, r2);
}
