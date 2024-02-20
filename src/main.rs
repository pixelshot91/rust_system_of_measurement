#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(inherent_associated_types)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(const_option)]

mod fraction;
mod zero;

use std::{
    fmt::Debug,
    fmt::Display,
    ops::{Add, Mul, Sub},
};

const SECONDS_IN_MINUTE: u32 = 60;

use fraction::Fraction;
// #[derive(Debug)]
#[derive(Clone, Copy)]
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
pub fn second(v: f64) -> Unit<{ Fraction::ONE }, { Fraction::ZERO }, { Fraction::ZERO }> {
    Unit::<{ Fraction::ONE }, { Fraction::ZERO }, { Fraction::ZERO }> { value: v }
}
pub fn meter(v: f64) -> Unit<{ Fraction::ZERO }, { Fraction::ONE }, { Fraction::ZERO }> {
    Unit::<{ Fraction::ZERO }, { Fraction::ONE }, { Fraction::ZERO }> { value: v }
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

// impl<const T1: Fraction, const L1: Fraction, const M1: Fraction, const T2: Fraction, const L2: Fraction, const M2: Fraction>
//     Div<Unit<T2, L2, M2>> for Unit<T1, L1, M1>
// where
//     Unit<{ T1 - T2 }, { L1 - L2 }, { M1 - M2 }>: Sized,
// {
//     type Output = Unit<{ T1 - T2 }, { L1 - L2 }, { M1 - M2 }>;

//     fn div(self, rhs: Unit<T2, L2, M2>) -> Self::Output {
//         Self::Output {
//             value: self.value / rhs.value,
//         }
//     }
// }

impl<const T: Fraction, const L: Fraction, const M: Fraction> Unit<T, L, M>
where
    Unit<{ T.const_div(2) }, { L.const_div(2) }, { M.const_div(2) }>: Sized,
{
    pub type SqrtOutput = Unit<{ T.const_div(2) }, { L.const_div(2) }, { M.const_div(2) }>;
    // type Outpsut = Unit<{T.const_div(2.0)}, L, M>;

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

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m", self.value)
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
    let t1 = second(2.0);
    println!("t1 in minutes = {}", t1.minute());

    let d1 = meter(4.0);
    let d2 = meter(3.0);

    let total_distance = d1 + d2;
    println!("total_distance = {}", total_distance);
    // assert_eq!(total_distance, meter(7.0));

    let area = d1 * d2;
    dbg!(area);
    area.m2();

    let volume = area * meter(5.0);
    dbg!(volume);

    let sqrt_d1 = d1.sqrt();
    dbg!(sqrt_d1);
    let sqrt_d2 = d2.sqrt();
    dbg!(sqrt_d2);

    let sqrt_mul = sqrt_d1 * sqrt_d2;
    dbg!(sqrt_mul);

    sqrt_mul.km();

    /* let speed = d1 / t1;
    dbg!(speed);

    dbg!(speed.m_s());
    dbg!(speed.km_h());

    dbg!(speed / 3.0.into()); */

    // let freq = unitless(1.0) / t1;
    // dbg!(freq);

    // let speed_2 = d2 * freq;
    // dbg!(speed_2);
}
