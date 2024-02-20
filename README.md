# System of Measurement

Currently support 3 base physical quantities: Time, Length and Mass

The struct `Unit<T,L,M>` can represent any arbitrary unit being a product of these three base unit, raise to a fractional exponent.

| Name                  | T  | L   | M |
|-----------------------|----|-----|---|
| Duration              | 1  |     |   |
| Frequency             | -1 |     |   |
| Length                | 1  |     |   |
| Area                  | 2  |     |   |
| Volume                | 3  |     |   |
| Speed                 | -1 | 1   |   |
| Root square of length |    | 1/2 |   |
| Mass                  |    |     | 1 |

## Example
```rust
fn work() {
    let d1 = meter(4.0);
    let d2 = meter(3.0);

    let total_distance = d1 + d2;
    println!("total_distance = {}", total_distance);
    assert_eq!(total_distance, meter(7.0));
}
```

Adding or subtracting two different quantities result in a compile time error:
```rust
fn compile_error() {
    let d1 = meter(4.0);
    let d2 = second(3.0);

    let total_distance = d1 + d2;
    println!("total_distance = {}", total_distance);
    assert_eq!(total_distance, meter(7.0));
}
```

```
error[E0308]: mismatched types
   --> src/main.rs:201:31
    |
201 |     let total_distance = d1 + d2;
    |                               ^^ expected `Fraction { numerator: 0, denominator: 1 }`, found `Fraction { numerator: 1, denominator: 1 }`
    |
    = note: expected constant `Fraction { numerator: 0, denominator: 1 }`
               found constant `Fraction { numerator: 1, denominator: 1 }`
```