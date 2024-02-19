use std::ops::Add;

#[derive(Debug)]
struct Distance {
    meter : f64
}

impl Distance {
const METERS_IN_MILES : f64 = 1609.0;

    fn meter(meter: f64) -> Distance {
        Distance { meter }
    }

    fn mile(mile: f64) -> Distance {
        Distance { meter: mile * Self::METERS_IN_MILES }
    }
}

impl Add for Distance {
    type Output = Distance;

    fn add(self, rhs: Self) -> Self::Output {
        Distance {
            meter: self.meter + rhs.meter
        }
    }
}

fn main() {
    let d1 = Distance::meter(4.0);
    let d2 = Distance::mile(2.0);

    let  total = d1 + d2;

    dbg!(total);
}
