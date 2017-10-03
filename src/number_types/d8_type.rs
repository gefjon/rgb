use std::num::Wrapping;
use number_types::d16_type::d16;

#[allow(warnings)]
#[derive(Copy, Clone, Debug)]
pub struct d8(pub Wrapping<u8>);

impl ::std::cmp::PartialEq for d8 {
    fn eq(&self, &d8(other): &Self) -> bool {
        let d8(me) = *self;
        me == other
    }
}

impl ::std::cmp::Eq for d8 {}

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

impl ::std::convert::Into<d16> for d8 {
    fn into(self) -> d16 {
        let d8(Wrapping(me)) = self;
        d16(Wrapping(me as u16))
    }
}
