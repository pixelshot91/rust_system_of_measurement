use std::{fmt::Display, marker::ConstParamTy, ops::Add};

use crate::zero::Zero;

#[derive(Debug, PartialEq, Eq, ConstParamTy)]
pub struct Fraction {
    numerator: i32,
    denominator: u32,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

impl Fraction {
    pub const ZERO: Fraction = Fraction {
        numerator: 0,
        denominator: 1,
    };
    pub const ONE: Fraction = Fraction {
        numerator: 1,
        denominator: 1,
    };

    pub const TWO: Fraction = Fraction {
        numerator: 2,
        denominator: 1,
    };
    pub const MINUS_ONE: Fraction = Fraction {
        numerator: -1,
        denominator: 1,
    };
    const fn new(numerator: i32, denominator: u32) -> Fraction {
        let gcd = gcd(numerator as u32, denominator);

        Fraction {
            numerator: numerator.checked_div(gcd as i32).unwrap(),
            denominator: denominator.checked_div(gcd).unwrap(),
        }
    }

    /// sum = n1 / d1 + n2 / d2
    /// let lcm = lcm(d1, d2)
    /// lcm = d1 * k1
    /// lcm = d2 * k2
    /// sum = ( n1 * k1) / lcm + (n2 * k2) / lcm
    ///     = (n1 * k1 + n2 * k2) / lcm
    /// lcm(a,b) * gcd(a,b) = a * b
    /// so lcm(a,b) = a * b / gcd(a,b)
    pub const fn const_add(self, rhs: Self) -> Self {
        let gcd = gcd(self.denominator, rhs.denominator);

        let lcm = self
            .denominator
            .checked_mul(rhs.denominator)
            .unwrap()
            .checked_div(gcd)
            .unwrap();

        let k1 = lcm.checked_div(self.denominator).unwrap();
        let k2 = lcm.checked_div(rhs.denominator).unwrap();
        let numerator = self
            .numerator
            .checked_mul(k1 as i32)
            .unwrap()
            .checked_add(rhs.numerator.checked_mul(k2 as i32).unwrap())
            .unwrap();
        Fraction::new(numerator, lcm)
    }

    pub const fn const_sub(self, rhs: Self) -> Self {
        let gcd = gcd(self.denominator, rhs.denominator);

        let lcm = self
            .denominator
            .checked_mul(rhs.denominator)
            .unwrap()
            .checked_div(gcd)
            .unwrap();

        let k1 = lcm.checked_div(self.denominator).unwrap();
        let k2 = lcm.checked_div(rhs.denominator).unwrap();
        let numerator = self
            .numerator
            .checked_mul(k1 as i32)
            .unwrap()
            .checked_sub(rhs.numerator.checked_mul(k2 as i32).unwrap())
            .unwrap();
        Fraction::new(numerator, lcm)
    }

    pub const fn const_div(self, div: u32) -> Self {
        Fraction::new(self.numerator, self.denominator.checked_mul(div).unwrap())
    }
}

impl From<i32> for Fraction {
    fn from(value: i32) -> Self {
        Fraction {
            numerator: value,
            denominator: 1,
        }
    }
}

const fn gcd(x: u32, y: u32) -> u32 {
    let mut x = x;
    let mut y = y;

    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

const fn gcd_generic<T: ~const PartialEq + std::ops::Rem<Output = T> + Zero<T> + Copy>(
    x: T,
    y: T,
) -> T {
    let mut x = x;
    let mut y = y;

    while y != T::zero() {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        self.const_add(rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::fraction::{gcd, gcd_generic};

    use super::Fraction;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(78, 2), 2);
        assert_eq!(gcd_generic(78, 2), 2);
    }

    #[test]
    fn my_add() {
        let f1 = Fraction::new(1, 1);
        let f2 = Fraction::new(1, 1);

        let sum = f1.const_add(f2);
        dbg!(&sum);
        assert_eq!(sum, Fraction::new(2, 1));
    }

    #[test]
    fn simplify_add() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 2);

        let sum = f1.const_add(f2);
        dbg!(&sum);
        assert_eq!(sum, Fraction::new(1, 1));
    }

    #[test]
    fn simplify_div() {
        let f1 = Fraction::new(6, 1);
        let r = f1.const_div(2);
        dbg!(&r);
        assert_eq!(r, Fraction::new(3, 1));
    }
    #[test]
    fn basic() {
        let f1 = Fraction::new(3, 2);
        let f2 = Fraction::new(5, 2);

        let sum = f1 + f2;
        assert_eq!(sum, Fraction::new(4, 1));
        assert_eq!(
            sum,
            Fraction {
                numerator: 4,
                denominator: 1
            }
        );
    }

    #[test]
    fn d2_is_multiple_of_d1() {
        let f1 = Fraction::new(3, 2);
        let f2 = Fraction::new(5, 4);

        let sum = f1 + f2;
        assert_eq!(sum, Fraction::new(11, 4));
    }
}
