extern crate creusot_std;
use creusot_std::{logic::float::FloatLogic, prelude::*};

// Named constants: float literals are not accepted inside contracts.
const F64_ZERO: f64 = 0.0;
const F64_BYTE_MAX: f64 = 255.0;
const F64_BOUND: f64 = 1.0e30;
const F64_NEGATIVE_BOUND: f64 = -1.0e30;

// Widening is exact: no obligation, and the real value is preserved.
#[ensures(x.is_finite_log() ==> result.is_finite_log() && result.to_real() == x.to_real())]
pub fn widen(x: f32) -> f64 {
    x as f64
}

// Narrowing a bounded value discharges `narrowing_in_range`.
#[requires(F64_NEGATIVE_BOUND <= x && x <= F64_BOUND)]
pub fn narrow_bounded(x: f64) -> f32 {
    x as f32
}

// Narrowing a constant discharges it without a precondition.
pub fn narrow_constant() -> f32 {
    1.5f64 as f32
}

// A float in the target's range discharges `is_finite /\ in_bounds (to_int)`.
#[requires(F64_ZERO <= x && x <= F64_BYTE_MAX)]
pub fn float_to_byte(x: f64) -> u8 {
    x as u8
}

// Every `i32` is exactly representable: no obligation.
#[ensures(result.is_finite_log())]
pub fn int_to_float(x: i32) -> f64 {
    x as f64
}

// `u128 as f32` is the one integer-to-float cast that can overflow.
#[requires(x@ < 1_000_000)]
pub fn small_u128_to_f32(x: u128) -> f32 {
    x as f32
}

// Logic-level casts lower to the total conversion, with no guard.
#[logic]
pub fn widen_logic(x: f32) -> f64 {
    x as f64
}

#[ensures(result == x.is_nan_log())]
pub fn nan_check(x: f64) -> bool {
    x.is_nan()
}

#[requires(!x.is_nan_log())]
#[ensures(!result.is_nan_log())]
pub fn abs_keeps_number(x: f64) -> f64 {
    x.abs()
}

#[requires(!a.is_nan_log() && !b.is_nan_log())]
#[ensures(!(result < a) && !(result < b))]
pub fn max_is_upper_bound(a: f64, b: f64) -> f64 {
    a.max(b)
}

#[ensures(result == x.sqrt_log())]
pub fn sqrt_is_ieee(x: f64) -> f64 {
    x.sqrt()
}
