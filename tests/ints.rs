extern crate first_explore;

use first_explore::utils::ints;

#[test]
fn larger_is_false_for_different() {
    assert!(!ints::is_larger(1, 5))
}

#[test]
fn larger_is_false_for_same() {
    assert!(!ints::is_larger(1, 1))
}

#[test]
fn larger_is_true() {
    assert!( ints::is_larger(5, 1))
}

#[test]
fn zero_mapped_to_zero() {
    assert_eq!(ints::linearscale(0), 0)
}

#[test]
fn max_mapped_to_255() {
    assert_eq!(ints::linearscale(std::u32::MAX), 255)
}

#[test]
fn middle_mapped_to_127() {
    assert_eq!(ints::linearscale(std::u32::MAX / 2), 127)
}