use std::num::Wrapping;
use number_types::d8_type::d8;
use number_types::a16_type::a16;

#[allow(Warnings)]
#[derive(Copy, Clone, Debug)]
pub struct a8(pub Wrapping<u8>);

impl ::std::convert::From<d8> for a8 {
    fn from(this: d8) -> Self {
        a8(this.0)
    }
}

impl ::std::convert::From<a8> for d8 {
    fn from(this: a8) -> Self {
        d8(this.0)
    }
}

impl ::std::convert::From<a8> for a16 {
    fn from(this: a8) -> Self {
        a16(Wrapping(((this.0).0 as u16) + 0xFF00))
    }
}
