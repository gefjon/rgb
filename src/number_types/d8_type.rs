use std::num::Wrapping;
use number_types::d16_type::d16;

#[allow(warnings)]
#[derive(Copy, Clone, Debug)]
pub struct d8(pub Wrapping<u8>);

const UPPER_NIBBLE_MASK: d8 = d8(Wrapping(0b11110000));
const LOWER_NIBBLE_MASK: d8 = d8(Wrapping(0b00001111));

impl d8 {
    pub fn upper_nibble(self) -> Self {
        self & UPPER_NIBBLE_MASK
    }

    pub fn lower_nibble(self) -> Self {
        self & LOWER_NIBBLE_MASK
    }

    pub const HIGHEST_BIT_MASK: d8 = d8(Wrapping(0b10000000));
    pub const LOWEST_BIT_MASK: d8 = d8(Wrapping(0b00000001));
    pub const ZERO: d8 = d8(Wrapping(0));
}

impl ::std::cmp::PartialEq for d8 {
    fn eq(&self, &d8(other): &Self) -> bool {
        let d8(me) = *self;
        me == other
    }
}

impl ::std::cmp::Eq for d8 {}

impl ::std::cmp::PartialEq<u8> for d8 {
    fn eq(&self, &other: &u8) -> bool {
        let d8(Wrapping(me)) = *self;
        me == other
    }
}

impl ::std::cmp::PartialOrd for d8 {
    fn partial_cmp(&self, &d8(other): &Self) -> Option<::std::cmp::Ordering> {
        let d8(me) = *self;
        if me < other {
            Some(::std::cmp::Ordering::Less)
        } else if me > other {
            Some(::std::cmp::Ordering::Greater)
        } else {
            Some(::std::cmp::Ordering::Equal)
        }
    }

    fn lt(&self, &d8(other): &Self) -> bool {
        let d8(me) = *self;
        me < other
    }

    fn gt(&self, &d8(other): &Self) -> bool {
        let d8(me) = *self;
        me > other
    }

    fn le(&self, &d8(other): &Self) -> bool {
        let d8(me) = *self;
        me <= other
    }

    fn ge(&self, &d8(other): &Self) -> bool {
        let d8(me) = *self;
        me >= other
    }
}

impl ::std::cmp::Ord for d8 {
    fn cmp(&self, &d8(other): &Self) -> ::std::cmp::Ordering {
        let d8(me) = *self;
        if me < other {
            ::std::cmp::Ordering::Less
        } else if me > other {
            ::std::cmp::Ordering::Greater
        } else {
            ::std::cmp::Ordering::Equal
        }
    }
}

impl ::std::ops::Add for d8 {
    type Output = Self;
    fn add(self, d8(other): Self) -> <Self as ::std::ops::Add<Self>>::Output {
        let d8(me) = self;
        d8(me + other)
    }
}

impl ::std::ops::AddAssign for d8 {
    fn add_assign(&mut self, d8(other): Self) {
        self.0 = self.0 + other;
    }
}

impl ::std::ops::Add<u8> for d8 {
    type Output = Self;
    fn add(self, other: u8) -> <Self as ::std::ops::Add<u8>>::Output {
        let d8(me) = self;
        d8(me + Wrapping(other))
    }
}

impl ::std::ops::AddAssign<u8> for d8 {
    fn add_assign(&mut self, other: u8) {
        self.0 = self.0 + Wrapping(other);
    }
}

impl ::std::ops::Sub for d8 {
    type Output = Self;
    fn sub(self, d8(other): Self) -> <Self as ::std::ops::Add<Self>>::Output {
        let d8(me) = self;
        d8(me - other)
    }
}

impl ::std::ops::SubAssign for d8 {
    fn sub_assign(&mut self, d8(other): Self) {
        self.0 = self.0 - other;
    }
}

impl ::std::ops::SubAssign<u8> for d8 {
    fn sub_assign(&mut self, other: u8) {
        self.0 = self.0 - Wrapping(other);
    }
}

impl ::std::ops::BitAnd for d8 {
    type Output = d8;
    fn bitand(self, d8(other): Self) -> <Self as ::std::ops::BitAnd>::Output {
        let d8(me) = self;
        d8(me & other)
    }
}

impl ::std::ops::BitAndAssign for d8 {
    fn bitand_assign(&mut self, d8(other): Self) {
        self.0 &= other;
    }
}

impl ::std::ops::Shl<usize> for d8 {
    type Output = Self;

    fn shl(self, rhs: usize) -> <Self as ::std::ops::Shl<usize>>::Output {
        let d8(me) = self;
        d8(me << rhs)
    }
}

impl ::std::ops::ShlAssign<usize> for d8 {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs;
    }
}

impl ::std::ops::ShrAssign<usize> for d8 {
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs;
    }
}

impl ::std::ops::Not for d8 {
    type Output = Self;

    fn not(self) -> <Self as ::std::ops::Not>::Output {
        let d8(me) = self;
        d8(!me)
    }
}

impl ::std::convert::Into<d16> for d8 {
    fn into(self) -> d16 {
        let d8(Wrapping(me)) = self;
        d16(Wrapping(me as u16))
    }
}
