use std::ops::{Deref, Index, Range};

/// Wrapper for extracted bit values that allows Deref to u8
#[derive(Clone, Copy)]
pub struct BitValue(u8);

impl Deref for BitValue {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}

pub struct Byte_view<'a> {
    data: &'a u8,
}

impl<'a> Byte_view<'a> {
    pub fn new(data: &'a u8) -> Self {
        Self { data: data }
    }
}

impl<'a> Deref for Byte_view<'a> {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}

/// Index with Range<usize> to extract bits using [start..end] syntax
impl<'a> Index<Range<usize>> for Byte_view<'a> {
    type Output = BitValue;

    fn index(&self, range: Range<usize>) -> &Self::Output {
        let shift = range.start as u8;
        let width = (range.end - range.start) as u8;
        let mask = (1u8 << width).wrapping_sub(1);
        let bits = (**self >> shift) & mask;
        Box::leak(Box::new(BitValue(bits)))
    }
}

#[test]
fn index_test() {
    let byte = 2_u8;
    let bits = Byte_view::new(&byte);
    let obits = bits[0..1];
    assert_eq!(*bits & 1, *obits)
}
