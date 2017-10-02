use std::num::Wrapping;
use number_types::d8_type::d8;

#[allow(warnings)]
#[derive(Copy, Clone, Debug)]
pub struct d16(pub Wrapping<u16>);

impl ::std::cmp::PartialEq for d16 {
    fn eq(&self, &d16(other): &Self) -> bool {
        let d16(me) = *self;
        me == other
    }
}

impl ::std::cmp::Eq for d16 {}

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
        use std::mem::transmute;
        
        unsafe {
            transmute(&self)
        }
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

