pub const COLOR_ZERO: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const COLOR_BLACK: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: u8::MAX,
};
pub const COLOR_WHITE: Pixel = Pixel {
    r: u8::MAX,
    g: u8::MAX,
    b: u8::MAX,
    a: u8::MAX,
};
pub const COLOR_RED: Pixel = Pixel {
    r: u8::MAX,
    g: 0,
    b: 0,
    a: u8::MAX,
};
pub const COLOR_GREEN: Pixel = Pixel {
    r: 0,
    g: u8::MAX,
    b: 0,
    a: u8::MAX,
};
pub const COLOR_BLUE: Pixel = Pixel {
    r: 0,
    g: 0,
    b: u8::MAX,
    a: u8::MAX,
};

#[repr(C)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
