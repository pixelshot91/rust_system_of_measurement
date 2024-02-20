use std::{marker::ConstParamTy, ops::Add};

#[derive(Debug, PartialEq, Eq, ConstParamTy)]
pub struct Fraction {
    numerator: i32,
    denominator: u32,
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
        let gcd = gcd(denominator, denominator);

        Fraction {
            numerator: numerator.checked_div(gcd as i32).unwrap(),
            denominator: denominator.checked_div(gcd).unwrap(),
        }
    }

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

    pub const fn const_div(self, div: u32) -> Self {
        Fraction {
            numerator: self.numerator,
            denominator: self.denominator.checked_mul(div).unwrap(),
        }
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
        // while y != T::zero() {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

/* const fn gcd<T: PartialEq + std::ops::Rem<Output = T> + Zero<T> + Copy>(x: T, y: T) -> T {
    let mut x = x;
    let mut y = y;
    let a = 4_u32;
    let b = 5_u32;

    let c = a % b;

    let t = x + y;

    while 2 != 4 {
        // while y != T::zero() {
        let t = y;
        y = x % y;
        x = t;
    }
    x
} */

/// sum = n1 / d1 + n2 / d2
/// let lcm = lcm(d1, d2)
/// lcm = d1 * k1
/// lcm = d2 * k2
/// sum = ( n1 * k1) / lcm + (n2 * k2) / lcm
///     = (n1 * k1 + n2 * k2) / lcm
/// lcm(a,b) * gcd(a,b) = a * b
/// so lcm(a,b) = a * b / gcd(a,b)
// #[const_trait]
impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
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
        Fraction {
            numerator,
            denominator: lcm,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Fraction;

    #[test]
    fn my_add() {
        let f1 = Fraction::new(1, 1);
        let f2 = Fraction::new(1, 1);

        let sum = f1.const_add(f2);
        dbg!(&sum);
        assert_eq!(sum, Fraction::new(2, 1));
    }

    #[test]
    fn simplify() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 2);

        let sum = f1.const_add(f2);
        dbg!(&sum);
        assert_eq!(sum, Fraction::new(1, 1));
    }
    #[test]
    fn basic() {
        let f1 = Fraction::new(3, 2);
        let f2 = Fraction::new(5, 2);

        let sum = f1 + f2;
        assert_eq!(sum, Fraction::new(8, 2));
    }

    #[test]
    fn d2_is_multiple_of_d1() {
        let f1 = Fraction::new(3, 2);
        let f2 = Fraction::new(5, 4);

        let sum = f1 + f2;
        assert_eq!(sum, Fraction::new(11, 4));
    }
}
