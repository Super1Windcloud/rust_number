use std::sync::atomic::{AtomicBool, Ordering};

const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;
const MIN_SAFE_INTEGER: f64 = -9_007_199_254_740_991.0;

static BOUNDARY_CHECKING_ENABLED: AtomicBool = AtomicBool::new(true);

#[derive(Clone, Debug, PartialEq)]
pub enum NumberInput {
    Float(f64),
    Text(String),
}

impl NumberInput {
    fn as_f64(&self) -> f64 {
        match self {
            Self::Float(value) => *value,
            Self::Text(value) => value.parse::<f64>().unwrap_or_else(|_| {
                panic!("failed to parse numeric input: {value}");
            }),
        }
    }

    fn as_string(&self) -> String {
        match self {
            Self::Float(value) => value.to_string(),
            Self::Text(value) => value.trim().to_string(),
        }
    }
}

macro_rules! impl_number_input_from_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl From<$ty> for NumberInput {
                fn from(value: $ty) -> Self {
                    Self::Float(value as f64)
                }
            }
        )*
    };
}

impl From<f64> for NumberInput {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<f32> for NumberInput {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<&str> for NumberInput {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for NumberInput {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl_number_input_from_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize);

#[derive(Debug)]
struct ParsedNumber {
    negative: bool,
    digits: String,
    decimal_pos: i32,
}

impl ParsedNumber {
    fn is_zero(&self) -> bool {
        self.digits.chars().all(|ch| ch == '0')
    }

    fn digit_length(&self) -> u32 {
        let len = self.digits.len() as i32 - self.decimal_pos;
        len.max(0) as u32
    }

    fn fixed_integer_string(&self) -> String {
        if self.decimal_pos <= 0 {
            format!(
                "{}{}",
                "0".repeat((-self.decimal_pos) as usize),
                self.digits
            )
        } else if self.decimal_pos as usize >= self.digits.len() {
            format!(
                "{}{}",
                self.digits,
                "0".repeat(self.decimal_pos as usize - self.digits.len())
            )
        } else {
            self.digits.clone()
        }
    }
}

fn parse_number<T: Into<NumberInput>>(num: T) -> ParsedNumber {
    let input: NumberInput = num.into();
    let value = input.as_string();
    let trimmed = value.trim();
    let (negative, unsigned) = match trimmed.as_bytes().first() {
        Some(b'-') => (true, &trimmed[1..]),
        Some(b'+') => (false, &trimmed[1..]),
        _ => (false, trimmed),
    };
    let mut exp_split = unsigned.split(['e', 'E']);
    let coefficient = exp_split.next().unwrap_or("0");
    let exponent = exp_split
        .next()
        .map(|value| {
            value
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("invalid exponent: {trimmed}"))
        })
        .unwrap_or(0);
    let mut coefficient_split = coefficient.split('.');
    let integer = coefficient_split.next().unwrap_or("");
    let fraction = coefficient_split.next().unwrap_or("");
    let digits = format!("{integer}{fraction}");
    let digits = if digits.is_empty() {
        "0".to_string()
    } else {
        digits
    };
    let decimal_pos = integer.len() as i32 + exponent;

    ParsedNumber {
        negative,
        digits,
        decimal_pos,
    }
}

fn strip_with_precision(value: f64, precision: usize) -> f64 {
    if value == 0.0 || !value.is_finite() {
        return value;
    }

    let digits = precision.max(1) as i32;
    let exponent = value.abs().log10().floor() as i32;
    let shift = digits - exponent - 1;

    if shift >= 0 {
        let factor = 10_f64.powi(shift);
        (value * factor).round() / factor
    } else {
        let factor = 10_f64.powi(-shift);
        (value / factor).round() * factor
    }
}

fn check_boundary(value: f64) {
    if BOUNDARY_CHECKING_ENABLED.load(Ordering::Relaxed)
        && !(MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&value)
    {
        eprintln!(
            "{value} is beyond boundary when transfer to integer, the results may not be accurate"
        );
    }
}

pub fn enable_boundary_checking(flag: bool) {
    BOUNDARY_CHECKING_ENABLED.store(flag, Ordering::Relaxed);
}

pub fn strip<T: Into<NumberInput>>(num: T, precision: usize) -> f64 {
    let input: NumberInput = num.into();
    strip_with_precision(input.as_f64(), precision)
}

pub fn digit_length<T: Into<NumberInput>>(num: T) -> u32 {
    parse_number(num).digit_length()
}

pub fn float2fixed<T: Into<NumberInput>>(num: T) -> i128 {
    let parsed = parse_number(num);
    if parsed.is_zero() {
        return 0;
    }

    let mut digits = parsed.fixed_integer_string();
    while digits.starts_with('0') && digits.len() > 1 {
        digits.remove(0);
    }

    let fixed = digits
        .parse::<i128>()
        .unwrap_or_else(|_| panic!("failed to convert into fixed integer: {digits}"));
    if parsed.negative { -fixed } else { fixed }
}

pub fn times<A: Into<NumberInput>, B: Into<NumberInput>>(num1: A, num2: B) -> f64 {
    let left: NumberInput = num1.into();
    let right: NumberInput = num2.into();
    let left_fixed = float2fixed(left.clone());
    let right_fixed = float2fixed(right.clone());
    let base_digits = digit_length(left) + digit_length(right);
    let raw = left_fixed * right_fixed;
    check_boundary(raw as f64);
    raw as f64 / 10_f64.powi(base_digits as i32)
}

pub fn plus<A: Into<NumberInput>, B: Into<NumberInput>>(num1: A, num2: B) -> f64 {
    let left: NumberInput = num1.into();
    let right: NumberInput = num2.into();
    let base = 10_f64.powi(digit_length(left.clone()).max(digit_length(right.clone())) as i32);
    (times(left, base) + times(right, base)) / base
}

pub fn minus<A: Into<NumberInput>, B: Into<NumberInput>>(num1: A, num2: B) -> f64 {
    let left: NumberInput = num1.into();
    let right: NumberInput = num2.into();
    let base = 10_f64.powi(digit_length(left.clone()).max(digit_length(right.clone())) as i32);
    (times(left, base) - times(right, base)) / base
}

pub fn divide<A: Into<NumberInput>, B: Into<NumberInput>>(num1: A, num2: B) -> f64 {
    let left: NumberInput = num1.into();
    let right: NumberInput = num2.into();
    let left_fixed = float2fixed(left.clone());
    let right_fixed = float2fixed(right.clone());
    check_boundary(left_fixed as f64);
    check_boundary(right_fixed as f64);

    times(
        left_fixed as f64 / right_fixed as f64,
        strip_with_precision(
            10_f64.powi(digit_length(right) as i32 - digit_length(left) as i32),
            15,
        ),
    )
}

pub fn round<T: Into<NumberInput>>(num: T, decimal: i32) -> f64 {
    let input: NumberInput = num.into();
    let value = input.as_f64();
    let base = 10_f64.powi(decimal);
    let mut result = divide(times(input, base).abs().round(), base);
    if value.is_sign_negative() && result != 0.0 {
        result = times(result, -1.0);
    }
    result
}

pub fn times_all<T>(nums: impl IntoIterator<Item = T>) -> f64
where
    T: Into<NumberInput>,
{
    let mut iter = nums.into_iter();
    let first = iter
        .next()
        .map(|value| value.into())
        .expect("times_all requires at least one value");
    iter.fold(first.as_f64(), |acc, next| times(acc, next))
}

pub fn plus_all<T>(nums: impl IntoIterator<Item = T>) -> f64
where
    T: Into<NumberInput>,
{
    let mut iter = nums.into_iter();
    let first = iter
        .next()
        .map(|value| value.into())
        .expect("plus_all requires at least one value");
    iter.fold(first.as_f64(), |acc, next| plus(acc, next))
}

pub fn minus_all<T>(nums: impl IntoIterator<Item = T>) -> f64
where
    T: Into<NumberInput>,
{
    let mut iter = nums.into_iter();
    let first = iter
        .next()
        .map(|value| value.into())
        .expect("minus_all requires at least one value");
    iter.fold(first.as_f64(), |acc, next| minus(acc, next))
}

pub fn divide_all<T>(nums: impl IntoIterator<Item = T>) -> f64
where
    T: Into<NumberInput>,
{
    let mut iter = nums.into_iter();
    let first = iter
        .next()
        .map(|value| value.into())
        .expect("divide_all requires at least one value");
    iter.fold(first.as_f64(), |acc, next| divide(acc, next))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_readme_examples() {
        assert_eq!(strip(0.09999999999999998_f64, 15), 0.1);
        assert_eq!(plus(0.1, 0.2), 0.3);
        assert_eq!(plus(2.3, 2.4), 4.7);
        assert_eq!(minus(1.0, 0.9), 0.1);
        assert_eq!(times(3, 0.3), 0.9);
        assert_eq!(times(0.362, 100), 36.2);
        assert_eq!(divide(1.21, 1.1), 1.1);
        assert_eq!(round(0.105, 2), 0.11);
    }

    #[test]
    fn supports_scientific_notation() {
        assert_eq!(digit_length(1.23e-5), 7);
        assert_eq!(float2fixed(1.23e-5), 123);
        assert_eq!(times("2.5e-3", "4e2"), 1.0);
        assert_eq!(divide("3e-4", "1.5e-2"), 0.02);
    }

    #[test]
    fn supports_string_inputs_and_multi_value_ops() {
        assert_eq!(plus("0.1", "0.2"), 0.3);
        assert_eq!(plus_all(["0.1", "0.2", "0.3"]), 0.6);
        assert_eq!(minus_all([1.0, 0.1, 0.2]), 0.7);
        assert_eq!(times_all([0.1, 0.2, 10.0]), 0.2);
        assert_eq!(divide_all([1.2, 0.3, 2.0]), 2.0);
    }

    #[test]
    fn rounds_negative_numbers_like_number_precision() {
        assert_eq!(round(-0.105, 2), -0.11);
        assert_eq!(round(-1.335, 2), -1.34);
    }

    #[test]
    fn boundary_check_toggle_is_configurable() {
        enable_boundary_checking(false);
        assert_eq!(times(9_007_199_254_740_991_u64, 1), 9_007_199_254_740_991.0);
        enable_boundary_checking(true);
    }
}
