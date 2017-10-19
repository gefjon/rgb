use std::num::Wrapping;
use number_types::d8_type::d8;
use number_types::d16_type::d16;

#[allow(warnings)]
#[derive(Copy, Clone, Debug)]
pub struct a16(pub Wrapping<u16>);

impl ::std::convert::From<d16> for a16 {
    fn from(this: d16) -> Self {
        a16(this.0)
    }
}

impl ::std::convert::From<a16> for d16 {
    fn from(this: a16) -> Self {
        d16(this.0)
    }
}

impl ::std::ops::Add<d8> for a16 {
    type Output = Self;
    fn add(self, d8(Wrapping(other)): d8) -> <Self as ::std::ops::Add<d8>>::Output {
        let a16(Wrapping(me)) = self;
        let other = Wrapping((other as i8) as i16);
        let me = Wrapping(me as i16);
        let result = (me + other).0;
        a16(Wrapping(result as u16))
    }
}

impl ::std::ops::Add for a16 {
    type Output = Self;
    fn add(self, a16(other): Self) -> <Self as ::std::ops::Add<Self>>::Output {
        a16(self.0 + other)
    }
}

impl ::std::ops::Add<u16> for a16 {
    type Output = Self;
    fn add(self, other: u16) -> <Self as ::std::ops::Add<u16>>::Output {
        a16(self.0 + Wrapping(other))
    }
}

impl ::std::ops::AddAssign<d8> for a16 {
    fn add_assign(&mut self, d8(Wrapping(rhs)): d8) {
        let lhs = (self.0).0;
        let lhs = Wrapping(lhs as i16);
        let rhs = Wrapping((rhs as i8) as i16);
        let result = (lhs + rhs).0;
        (self.0).0 = result as _;
    }
}
        
