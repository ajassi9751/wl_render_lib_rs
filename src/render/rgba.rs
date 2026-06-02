#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[test]
fn rgba_partialeq_default_test () {
    let r1 = Rgba::default();
    let r2 = Rgba { r: 0, g: 0, b:0, a: 0 };
    assert_eq!(r1, r2);
}