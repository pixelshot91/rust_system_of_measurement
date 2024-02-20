#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(const_option)]
#![feature(const_refs_to_cell)]

mod fraction;
mod zero;

use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

const SECONDS_IN_MINUTE: u32 = 60;

use fraction::Fraction;
// #[derive(Debug)]
#[derive(Clone, Copy, PartialEq)]
pub struct Unit<const T: Fraction, const L: Fraction, const M: Fraction> {
    value: f64,
}

type Duration = Unit<{ Fraction::ONE }, { Fraction::ZERO }, { Fraction::ZERO }>;

type Length = Unit<{ Fraction::ZERO }, { Fraction::ONE }, { Fraction::ZERO }>;
type Area = Unit<{ Fraction::ZERO }, { Fraction::TWO }, { Fraction::ZERO }>;

type Speed = Unit<{ Fraction::MINUS_ONE }, { Fraction::ONE }, { Fraction::ZERO }>;

pub fn unitless(v: f64) -> Unit<{ Fraction::ZERO }, { Fraction::ZERO }, { Fraction::ZERO }> {
    Unit::<{ Fraction::ZERO }, { Fraction::ZERO }, { Fraction::ZERO }> { value: v }
}
pub fn second(v: f64) -> Duration {
    Duration { value: v }
}
pub fn meter(v: f64) -> Length {
    Length { value: v }
}
pub fn meter_square(v: f64) -> Area {
    Area { value: v }
}
pub fn kg(v: f64) -> Unit<{ Fraction::ZERO }, { Fraction::ZERO }, { Fraction::ONE }> {
    Unit::<{ Fraction::ZERO }, { Fraction::ZERO }, { Fraction::ONE }> { value: v }
}

impl From<f64> for Unit<{ Fraction::ZERO }, { Fraction::ZERO }, { Fraction::ZERO }> {
    fn from(value: f64) -> Self {
        unitless(value)
    }
}

impl<const T: Fraction, const L: Fraction, const M: Fraction> Debug for Unit<T, L, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Unit")
            .field("T", &T)
            .field("L", &L)
            .field("M", &M)
            .field("value", &self.value)
            .finish()
    }
}

impl<const T: Fraction, const L: Fraction, const M: Fraction> Add<Unit<T, L, M>> for Unit<T, L, M> {
    type Output = Unit<T, L, M>;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<const T: Fraction, const L: Fraction, const M: Fraction> Sub<Unit<T, L, M>> for Unit<T, L, M> {
    type Output = Unit<T, L, M>;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<
        const T1: Fraction,
        const L1: Fraction,
        const M1: Fraction,
        const T2: Fraction,
        const L2: Fraction,
        const M2: Fraction,
    > Mul<Unit<T2, L2, M2>> for Unit<T1, L1, M1>
where
    Unit<{ T1.const_add(T2) }, { L1.const_add(L2) }, { M1.const_add(M2) }>: Sized,
{
    type Output = Unit<{ T1.const_add(T2) }, { L1.const_add(L2) }, { M1.const_add(M2) }>;

    fn mul(self, rhs: Unit<T2, L2, M2>) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

impl<
        const T1: Fraction,
        const L1: Fraction,
        const M1: Fraction,
        const T2: Fraction,
        const L2: Fraction,
        const M2: Fraction,
    > Div<Unit<T2, L2, M2>> for Unit<T1, L1, M1>
where
    Unit<{ T1.const_sub(T2) }, { L1.const_sub(L2) }, { M1.const_sub(M2) }>: Sized,
{
    type Output = Unit<{ T1.const_sub(T2) }, { L1.const_sub(L2) }, { M1.const_sub(M2) }>;

    fn div(self, rhs: Unit<T2, L2, M2>) -> Self::Output {
        Self::Output {
            value: self.value / rhs.value,
        }
    }
}

impl<const T: Fraction, const L: Fraction, const M: Fraction> Unit<T, L, M>
where
    Unit<{ T.const_div(2) }, { L.const_div(2) }, { M.const_div(2) }>: Sized,
{
    pub type SqrtOutput = Unit<{ T.const_div(2) }, { L.const_div(2) }, { M.const_div(2) }>;

    pub fn sqrt(&self) -> Self::SqrtOutput {
        Self::SqrtOutput {
            value: self.value.sqrt(),
        }
    }
}

impl Duration {
    pub fn minute(&self) -> f64 {
        self.value / (SECONDS_IN_MINUTE as f64)
    }
}

impl<const T: Fraction, const L: Fraction, const M: Fraction> Display for Unit<T, L, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if T == Fraction::ZERO && L == Fraction::ONE && M == Fraction::ZERO {
            write!(f, "{} m", &self.value)
        } else {
            write!(f, "{} s^({}) m^({}) kg^({})", &self.value, T, L, M)
        }
    }
}

impl Length {
    pub fn m(&self) -> f64 {
        self.value
    }

    pub fn km(&self) -> f64 {
        self.value / 1000.0
    }
}

impl Area {
    pub fn m2(&self) -> f64 {
        self.value
    }
}

impl Speed {
    pub fn m_s(&self) -> f64 {
        self.value
    }

    pub fn km_h(&self) -> f64 {
        self.value / 1000.0 * 3600.0
    }
}

fn main() {
    let t1 = second(120.0);
    println!("t1 in minutes = {}", t1.minute());

    let d1 = meter(4.0);
    let d2 = meter(3.0);

    let total_distance = d1 + d2;
    println!("total_distance = {}", total_distance);
    assert_eq!(total_distance, meter(7.0));

    let area = d1 * d2;
    assert_eq!(area.value, 12.0);
    assert_eq!(area, meter_square(12.0));

    let volume = area * meter(5.0);
    assert_eq!(volume.value, 60.0);

    let sqrt_d1 = d1.sqrt();
    println!("sqrt_d1 = {}", &sqrt_d1);
    let sqrt_d2 = d2.sqrt();
    println!("sqrt_d2 = {}", &sqrt_d2);

    let sqrt_mul = sqrt_d1 * sqrt_d2;
    println!("sqrt_mul = {}", &sqrt_mul);
    println!("sqrt_mul in km = {}", &sqrt_mul.km());

    let speed = meter(10.0) / second(1.0);
    dbg!(speed);

    dbg!(speed.m_s());
    dbg!(speed.km_h());

    let freq = unitless(1.0) / t1;
    dbg!(freq);

    let speed_2 = d2 * freq;
    dbg!(speed_2);
}
