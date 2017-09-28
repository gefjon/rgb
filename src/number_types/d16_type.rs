use std::num::Wrapping;
use number_types::d8_type::d8;

#[derive(Copy, Clone, Debug)]
pub struct d16(pub Wrapping<u16>);

impl ::std::cmp::PartialEq for d16 {
    fn eq(&self, other: &Self) -> bool {
        let d16(x) = *self;
        let d16(y) = *other;
        x == y
    }
}

impl ::std::convert::AsRef<[d8; 2]> for d16 {
    fn as_ref(&self) -> &[d8; 2] {
        use std::mem::transmute;
        
        unsafe {
            transmute(&self)
        }
    }
}
