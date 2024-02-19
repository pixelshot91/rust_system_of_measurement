#![feature(generic_const_exprs)]

use std::{
    fmt::Debug,
    ops::{Add, Mul},
};
// #[derive(Debug)]
#[derive(Clone, Copy)]
struct Length<const D: i32> {
    value: f64,
}

fn distance(v: f64) -> Length<1> {
    Length::<1> { value: v }
}

impl<const D: i32> Debug for Length<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Length")
            .field("D", &D)
            .field("value", &self.value)
            .finish()
    }
}

impl<const D: i32> Add<Length<D>> for Length<D> {
    type Output = Length<D>;

    fn add(self, rhs: Length<D>) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<const D1: i32, const D2: i32> Mul<Length<D2>> for Length<D1>
where
    Length<{ D1 + D1 }>: Sized,
{
    type Output = Length<{ D1 + D1 }>;

    fn mul(self, rhs: Length<D2>) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

fn main() {
    let d1 = distance(4.0);
    let d2 = distance(3.0);

    let total_distance = d1 + d2;

    dbg!(total_distance);

    let area = d1 * d2;
    dbg!(area);


    let volume = area * distance(5.0);
    dbg!(volume);
}

mod first {
    use std::ops::Add;

    #[derive(Debug)]
    struct Distance {
        meter: f64,
    }

    impl Distance {
        const METERS_IN_MILES: f64 = 1609.0;

        fn meter(meter: f64) -> Distance {
            Distance { meter }
        }

        fn mile(mile: f64) -> Distance {
            Distance {
                meter: mile * Self::METERS_IN_MILES,
            }
        }
    }

    impl Add for Distance {
        type Output = Distance;

        fn add(self, rhs: Self) -> Self::Output {
            Distance {
                meter: self.meter + rhs.meter,
            }
        }
    }

    fn main() {
        let d1 = Distance::meter(4.0);
        let d2 = Distance::mile(2.0);

        let total = d1 + d2;

        dbg!(total);
    }
}
