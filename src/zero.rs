pub trait Zero<T> {
  fn zero() -> T;
}

impl Zero<u32> for u32 {
    fn zero() -> u32 {
        0
    }
}