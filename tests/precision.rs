use rust_number::{
    digit_length, divide, divide_all, enable_boundary_checking, float2fixed, minus, minus_all,
    plus, plus_all, round, strip, times, times_all,
};

#[test]
fn readme_examples_work() {
    assert_eq!(strip(0.09999999999999998_f64, 15), 0.1);
    assert_eq!(plus(0.1, 0.2), 0.3);
    assert_eq!(minus(1.0, 0.9), 0.1);
    assert_eq!(times(3, 0.3), 0.9);
    assert_eq!(divide(1.21, 1.1), 1.1);
    assert_eq!(round(0.105, 2), 0.11);
}

#[test]
fn scientific_notation_is_supported() {
    assert_eq!(digit_length(1.23e-5), 7);
    assert_eq!(float2fixed(1.23e-5), 123);
    assert_eq!(times("2.5e-3", "4e2"), 1.0);
    assert_eq!(divide("3e-4", "1.5e-2"), 0.02);
}

#[test]
fn string_and_iter_inputs_are_supported() {
    assert_eq!(plus("0.1", "0.2"), 0.3);
    assert_eq!(plus_all(["0.1", "0.2", "0.3"]), 0.6);
    assert_eq!(minus_all([1.0, 0.1, 0.2]), 0.7);
    assert_eq!(times_all([0.1, 0.2, 10.0]), 0.2);
    assert_eq!(divide_all([1.2, 0.3, 2.0]), 2.0);
}

#[test]
fn negative_rounding_matches_number_precision() {
    assert_eq!(round(-0.105, 2), -0.11);
    assert_eq!(round(-1.335, 2), -1.34);
}

#[test]
fn boundary_checking_can_be_toggled() {
    enable_boundary_checking(false);
    assert_eq!(times(9_007_199_254_740_991_u64, 1), 9_007_199_254_740_991.0);
    enable_boundary_checking(true);
}
