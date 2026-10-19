//! Specifications for the IEEE-defined methods of `f32` and `f64`.
//!
//! Only methods whose result IEEE 754 defines exactly are specified here, each
//! stated through [`FloatLogic`]. Every extern spec is trusted, so each
//! postcondition must hold for *every* result Rust permits: orderings use IEEE
//! `<` only, which stays true whichever sign of zero an operation returns.
//!
//! Deliberately absent: `exp`, `ln`, `tanh` and the other math-library
//! functions. Rust documents their precision as non-deterministic — it may
//! differ even between two calls in one execution — so no deterministic
//! contract is true of them. A crate that needs to reason about their results
//! states its own extern specs as explicit assumptions; a specification here
//! would make that impossible, since a function carries only one.
//!
//! Also absent: the lossless `From` conversions into floats, such as
//! `From<i32> for f64`. With an extern spec on that impl, a fresh build of the
//! test suite panicked in 360 of 488 crates while loading this library's
//! metadata (`decode_def_id`: "Cannot resolve crate"); without it, the same
//! fresh build is clean. `core` defines these impls as `const impl`s, which
//! `extern_spec!` does not model — the likely cause, not a confirmed one. The
//! `as` casts these impls perform are specified by the cast lowering itself.
#![cfg_attr(not(creusot), allow(unused_imports))]
use crate::{
    logic::{float::FloatLogic, real::Real},
    prelude::*,
};

/// The methods whose specifications read the same for both widths, and the
/// finite associated constants.
macro_rules! spec_float {
    ($t:ident) => {
        extern_spec! {
            // Evaluated, like the integer `MIN`/`MAX`: `core` has no body for
            // these to translate, and without this Creusot aborts looking for
            // one. `INFINITY` and `NAN` are deliberately absent — the literal
            // printer only emits finite values.
            #[constant(eval)]
            const $t::EPSILON;

            #[constant(eval)]
            const $t::MIN;

            #[constant(eval)]
            const $t::MAX;

            #[constant(eval)]
            const $t::MIN_POSITIVE;

            impl $t {
                /// Exactly the NaN class.
                #[allow(dead_code)]
                #[ensures(result == self.is_nan_log())]
                fn is_nan(self) -> bool;

                /// Neither infinite nor NaN.
                #[allow(dead_code)]
                #[ensures(result == self.is_finite_log())]
                fn is_finite(self) -> bool;

                /// IEEE absolute value. Exact.
                #[allow(dead_code)]
                #[ensures(result == self.abs_log())]
                fn abs(self) -> $t;

                /// IEEE `maximumNumber` on NaNs, `maxNum` on signed zeros, as
                /// Rust documents. Both NaN gives a NaN whose payload Rust picks
                /// by its arithmetic rules, so no equality with either input is
                /// claimed there. Otherwise the result is one of the arguments —
                /// either one when they compare equal, as for `+0` and `-0` —
                /// the non-NaN one when exactly one is NaN, and not IEEE-less
                /// than either.
                #[allow(dead_code)]
                #[ensures(self.is_nan_log() && other.is_nan_log() ==> result.is_nan_log())]
                #[ensures(!(self.is_nan_log() && other.is_nan_log())
                    ==> result == self || result == other)]
                #[ensures(self.is_nan_log() && !other.is_nan_log() ==> result == other)]
                #[ensures(!self.is_nan_log() && other.is_nan_log() ==> result == self)]
                #[ensures(!self.is_nan_log() && !other.is_nan_log()
                    ==> !(result < self) && !(result < other))]
                fn max(self, other: $t) -> $t;
            }
        }
    };
}

spec_float!(f32);
spec_float!(f64);

/// The methods defined in `std` rather than `core`, so only present — and only
/// specifiable — when this crate is built with its `std` feature.
macro_rules! spec_float_std {
    ($t:ident) => {
        extern_spec! {
            impl $t {
                /// Rounding to integral, ties to even. Exact. A finite `self`
                /// lies within a half of the result: IEEE 754's
                /// `roundToIntegralTiesToEven` picks a nearest integer, which
                /// Why3 axiomatizes only for ties away from zero, and which the
                /// solvers do not derive from the float theory.
                #[allow(dead_code)]
                #[ensures(result == self.round_ties_even_log())]
                #[ensures(self.is_finite_log() ==> Real::from_int(2) * result.to_real()
                    - Real::from_int(1) <= Real::from_int(2) * self.to_real())]
                #[ensures(self.is_finite_log() ==> Real::from_int(2) * self.to_real()
                    <= Real::from_int(2) * result.to_real() + Real::from_int(1))]
                fn round_ties_even(self) -> $t;

                /// Correctly rounded square root. Rust documents the result as
                /// "the rounded infinite-precision result", specified by IEEE 754
                /// as `squareRoot` and "guaranteed not to change".
                #[allow(dead_code)]
                #[ensures(result == self.sqrt_log())]
                fn sqrt(self) -> $t;

                /// `self · a + b` with a single rounding: IEEE
                /// `fusedMultiplyAdd`. Rust documents `mul_add` as computing
                /// `(self * a) + b` with only one rounding error.
                #[allow(dead_code)]
                #[ensures(result == self.mul_add_log(a, b))]
                fn mul_add(self, a: $t, b: $t) -> $t;
            }
        }
    };
}

#[cfg(feature = "std")]
spec_float_std!(f32);
#[cfg(feature = "std")]
spec_float_std!(f64);
