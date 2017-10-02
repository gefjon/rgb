use std::num::Wrapping;
use number_types::d16_type::d16;

#[allow(warnings)]
#[derive(Copy, Clone, Debug)]
pub struct d8(pub Wrapping<u8>);

impl ::std::cmp::PartialEq for d8 {
    fn eq(&self, other: &Self) -> bool {
        let d8(x) = *self;
        let d8(y) = *other;
        x == y
    }
}
