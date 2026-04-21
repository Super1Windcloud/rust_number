# rust_number

`rust_number` is a Rust port of [`number-precision`](https://github.com/nefe/number-precision) for safer decimal arithmetic with `f64`.

It helps avoid common floating-point surprises such as:

```text
0.1 + 0.2 != 0.3
1.0 - 0.9 != 0.1
3.0 * 0.3 != 0.9
```

## Features

- Corrects common precision issues for addition, subtraction, multiplication, division, and rounding
- Supports decimal strings and scientific notation
- Exposes helper utilities like `strip`, `digit_length`, and `float2fixed`
- Includes iterator-based helpers for multi-value operations

## Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
rust_number = "0.1.0"
```

## Quick Start

```rust
use rust_number::{divide, minus, plus, round, strip, times};

fn main() {
    assert_eq!(strip(0.09999999999999998_f64, 15), 0.1);
    assert_eq!(plus(0.1, 0.2), 0.3);
    assert_eq!(minus(1.0, 0.9), 0.1);
    assert_eq!(times(3, 0.3), 0.9);
    assert_eq!(divide(1.21, 1.1), 1.1);
    assert_eq!(round(0.105, 2), 0.11);
}
```

Run the example locally:

```bash
cargo run --example basic_usage
```

## API

- `strip(num, precision) -> f64`
- `digit_length(num) -> u32`
- `float2fixed(num) -> i128`
- `plus(a, b) -> f64`
- `minus(a, b) -> f64`
- `times(a, b) -> f64`
- `divide(a, b) -> f64`
- `round(num, decimal) -> f64`
- `plus_all(values) -> f64`
- `minus_all(values) -> f64`
- `times_all(values) -> f64`
- `divide_all(values) -> f64`
- `enable_boundary_checking(enabled)`

All numeric APIs accept:

- integer types
- `f32` / `f64`
- `&str`
- `String`

## Multi-Value Operations

```rust
use rust_number::{divide_all, plus_all, times_all};

fn main() {
    assert_eq!(plus_all([0.1, 0.2, 0.3]), 0.6);
    assert_eq!(times_all([0.1, 0.2, 10.0]), 0.2);
    assert_eq!(divide_all([1.2, 0.3, 2.0]), 2.0);
}
```

## Scientific Notation

```rust
use rust_number::{digit_length, divide, float2fixed, times};

fn main() {
    assert_eq!(digit_length(1.23e-5), 7);
    assert_eq!(float2fixed(1.23e-5), 123);
    assert_eq!(times("2.5e-3", "4e2"), 1.0);
    assert_eq!(divide("3e-4", "1.5e-2"), 0.02);
}
```

## Testing

```bash
cargo test
```

## License

MIT
