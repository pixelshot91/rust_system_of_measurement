#![feature(generic_const_exprs)]

use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};
// #[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Unit<const T: i32, const L: i32, const M: i32> {
    value: f64,
}

pub fn unitless(v: f64) -> Unit<0, 0, 0> {
    Unit::<0, 0, 0> { value: v }
}
pub fn second(v: f64) -> Unit<1, 0, 0> {
    Unit::<1, 0, 0> { value: v }
}
pub fn meter(v: f64) -> Unit<0, 1, 0> {
    Unit::<0, 1, 0> { value: v }
}
pub fn kg(v: f64) -> Unit<0, 1, 0> {
    Unit::<0, 1, 0> { value: v }
}

impl From<f64> for Unit<0,0,0> {
    fn from(value: f64) -> Self {
        unitless(value)
    }
}

impl<const T: i32, const L: i32, const M: i32> Debug for Unit<T, L, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Unit")
            .field("T", &T)
            .field("L", &L)
            .field("M", &M)
            .field("value", &self.value)
            .finish()
    }
}

impl<const T: i32, const L: i32, const M: i32> Add<Unit<T, L, M>> for Unit<T, L, M> {
    type Output = Unit<T, L, M>;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<const T: i32, const L: i32, const M: i32> Sub<Unit<T, L, M>> for Unit<T, L, M> {
    type Output = Unit<T, L, M>;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<const T1: i32, const L1: i32, const M1: i32, const T2: i32, const L2: i32, const M2: i32>
    Mul<Unit<T2, L2, M2>> for Unit<T1, L1, M1>
where
    Unit<{ T1 + T2 }, { L1 + L2 }, { M1 + M2 }>: Sized,
{
    type Output = Unit<{ T1 + T2 }, { L1 + L2 }, { M1 + M2 }>;

    fn mul(self, rhs: Unit<T2, L2, M2>) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

impl<const T1: i32, const L1: i32, const M1: i32, const T2: i32, const L2: i32, const M2: i32>
    Div<Unit<T2, L2, M2>> for Unit<T1, L1, M1>
where
    Unit<{ T1 - T2 }, { L1 - L2 }, { M1 - M2 }>: Sized,
{
    type Output = Unit<{ T1 - T2 }, { L1 - L2 }, { M1 - M2 }>;

    fn div(self, rhs: Unit<T2, L2, M2>) -> Self::Output {
        Self::Output {
            value: self.value / rhs.value,
        }
    }
}

impl Unit<-1, 1, 0> {
    pub fn m_s(&self) -> f64 {
        self.value
    }

    pub fn km_h(&self) -> f64 {
        self.value / 1000.0 * 3600.0
    }
}

fn main() {
    let t1 = second(2.0);
    let d1 = meter(4.0);
    let d2 = meter(3.0);

    let total_distance = d1 + d2;

    dbg!(total_distance);

    let area = d1 * d2;
    dbg!(area);

    let volume = area * meter(5.0);
    dbg!(volume);

    let speed = d1 / t1;
    dbg!(speed);

    dbg!(speed.m_s());
    dbg!(speed.km_h());

    dbg!(speed / 3.0.into());

    let freq = unitless(1.0) / t1;
    dbg!(freq);

    let speed_2 = d2 * freq;
    dbg!(speed_2);
}
