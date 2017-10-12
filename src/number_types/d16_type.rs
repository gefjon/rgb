use std::num::Wrapping;
use number_types::d8_type::d8;

#[allow(warnings)]
#[derive(Copy, Clone, Debug)]
pub struct d16(pub Wrapping<u16>);

impl d16 {
    pub fn lsb(self) -> d8 {
        // remember, on little-endian systems, the lsb has index 0!
        let array: [d8; 2] = self.into();
        array[0]
    }

    pub fn least_significant_nibble(self) -> d8 {
        self.lsb().lower_nibble()
    }

    pub fn check_nibble_overflow(lhs: Self, rhs: Self) -> bool {
        let rhs: Wrapping<u8> = rhs.least_significant_nibble().into();
        let lhs: Wrapping<u8> = lhs.least_significant_nibble().into();
        ((rhs + lhs) > Wrapping(1 << 3))
    }

    pub fn add_and_check_overflow(lhs: Self, rhs: Self) -> (Self, bool) {
        let rhs: Wrapping<u32> = rhs.into();
        let lhs: Wrapping<u32> = lhs.into();
        let result = lhs + rhs;
        (result.into(), (result > Wrapping(::std::u16::MAX as _)))
    }

    pub const HIGHEST_BIT_MASK: d16 = d16(Wrapping(0b1000000000000000));
    pub const LOWEST_BIT_MASK: d16 = d16(Wrapping(0b0000000000000001));
    pub const ZERO: d16 = d16(Wrapping(0));
}

impl ::std::cmp::PartialEq for d16 {
    fn eq(&self, &d16(other): &Self) -> bool {
        let d16(me) = *self;
        me == other
    }
}

impl ::std::cmp::Eq for d16 {}

impl ::std::cmp::PartialEq<u16> for d16 {
    fn eq(&self, &other: &u16) -> bool {
        let d16(Wrapping(me)) = *self;
        me == other
    }
}

impl ::std::cmp::PartialOrd for d16 {
    fn partial_cmp(&self, &d16(other): &Self) -> Option<::std::cmp::Ordering> {
        let d16(me) = *self;
        if me < other {
            Some(::std::cmp::Ordering::Less)
        } else if me > other {
            Some(::std::cmp::Ordering::Greater)
        } else {
            Some(::std::cmp::Ordering::Equal)
        }
    }

    fn lt(&self, &d16(other): &Self) -> bool {
        let d16(me) = *self;
        me < other
    }

    fn gt(&self, &d16(other): &Self) -> bool {
        let d16(me) = *self;
        me > other
    }

    fn le(&self, &d16(other): &Self) -> bool {
        let d16(me) = *self;
        me <= other
    }

    fn ge(&self, &d16(other): &Self) -> bool {
        let d16(me) = *self;
        me >= other
    }
}

impl ::std::cmp::Ord for d16 {
    fn cmp(&self, &d16(other): &Self) -> ::std::cmp::Ordering {
        let d16(me) = *self;
        if me < other {
            ::std::cmp::Ordering::Less
        } else if me > other {
            ::std::cmp::Ordering::Greater
        } else {
            ::std::cmp::Ordering::Equal
        }
    }
}

impl ::std::convert::AsRef<[d8; 2]> for d16 {
    fn as_ref(&self) -> &[d8; 2] {
        unsafe {
            ::std::mem::transmute(&self)
        }
    }
}

impl ::std::convert::AsRef<d16> for [d8; 2] {
    fn as_ref(&self) -> &d16 {
        unsafe {
            ::std::mem::transmute(&self)
        }
    }
}

impl ::std::convert::AsMut<[d8; 2]> for d16 {
    fn as_mut(&mut self) -> &mut [d8; 2] {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl ::std::convert::AsMut<d16> for [d8; 2] {
    fn as_mut(&mut self) -> &mut d16 {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl ::std::convert::From<[d8; 2]> for d16 {
    fn from(this: [d8; 2]) -> Self {
        unsafe {
            ::std::mem::transmute(this)
        }
    }
}

impl ::std::convert::From<d16> for [d8; 2] {
    fn from(this: d16) -> Self {
        unsafe {
            ::std::mem::transmute(this)
        }
    }
}

impl ::std::convert::From<d16> for Wrapping<u32> {
    fn from(this: d16) -> Self {
        let d16(Wrapping(this)) = this;
        Wrapping(this as _)
    }
}

impl ::std::convert::From<Wrapping<u32>> for d16 {
    fn from(this: Wrapping<u32>) -> Self {
        let Wrapping(this) = this;
        d16(Wrapping(this as _))
    }
}

impl ::std::ops::Add for d16 {
    type Output = Self;
    fn add(self, d16(other): Self) -> <Self as ::std::ops::Add<Self>>::Output {
        let d16(me) = self;
        d16(me + other)
    }
}

impl ::std::ops::AddAssign for d16 {
    fn add_assign(&mut self, d16(other): Self) {
        self.0 = self.0 + other;
    }
}

impl ::std::ops::Add<u16> for d16 {
    type Output = Self;
    fn add(self, other: u16) -> <Self as ::std::ops::Add<u16>>::Output {
        let d16(me) = self;
        d16(me + Wrapping(other))
    }
}

impl ::std::ops::AddAssign<u16> for d16 {
    fn add_assign(&mut self, other: u16) {
        self.0 = self.0 + Wrapping(other);
    }
}

impl ::std::ops::Sub for d16 {
    type Output = Self;
    fn sub(self, d16(other): Self) -> <Self as ::std::ops::Sub<Self>>::Output {
        let d16(me) = self;
        d16(me - other)
    }
}

impl ::std::ops::SubAssign for d16 {
    fn sub_assign(&mut self, d16(other): Self) {
        self.0 = self.0 - other;
    }
}

impl ::std::ops::SubAssign<u16> for d16 {
    fn sub_assign(&mut self, other: u16) {
        self.0 = self.0 - Wrapping(other);
    }
}

impl ::std::ops::BitAnd for d16 {
    type Output = Self;
    fn bitand(self, d16(other): Self) -> <Self as ::std::ops::BitAnd>::Output {
        let d16(me) = self;
        d16(me & other)
    }
}

impl ::std::ops::BitAndAssign for d16 {
    fn bitand_assign(&mut self, d16(other): Self) {
        self.0 &= other;
    }
}
