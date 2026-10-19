//! The logic view of IEEE binary floats: the real number a finite float
//! denotes, the classification predicates that say when that number exists,
//! and the exact IEEE operations the `std` float specifications are stated in.
//!
//! Deliberately not a [`View`](crate::model::View): `@` promises a total model,
//! and a float denotes no real number when it is infinite or NaN. Every
//! statement made through [`FloatLogic::to_real`] therefore carries
//! [`FloatLogic::is_finite_log`] as a hypothesis, rather than leaving the
//! non-finite case quietly unconstrained.
use crate::{
    logic::{
        ops::{AddLogic, DivLogic, MulLogic, NegLogic, SubLogic},
        real::Real,
    },
    prelude::*,
};

mod private {
    pub trait _Sealed {}
    impl _Sealed for f32 {}
    impl _Sealed for f64 {}
}

/// Logic operations on `f32` and `f64`, each bound to the prelude module of its
/// width. Sealed: the builtins exist for exactly these two types.
///
/// A trait because Rust forbids inherent impls on primitive types outside
/// `std` — the same reason [`PartialOrdLogic`] is implemented for `f64`.
pub trait FloatLogic: private::_Sealed + Sized {
    /// The real number this float denotes. Unconstrained on infinities and NaN,
    /// so every use states [`FloatLogic::is_finite_log`] alongside it.
    #[logic]
    fn to_real(self) -> Real;

    /// Neither infinite nor NaN. Exactly one of this, `is_infinite_log` and
    /// `is_nan_log` holds.
    #[logic]
    fn is_finite_log(self) -> bool;

    /// NaN, of any payload; payloads are not modelled.
    #[logic]
    fn is_nan_log(self) -> bool;

    /// Positive or negative infinity.
    #[logic]
    fn is_infinite_log(self) -> bool;

    /// Zero, of either sign.
    #[logic]
    fn is_zero_log(self) -> bool;

    /// IEEE absolute value. Exact: clears the sign.
    #[logic]
    fn abs_log(self) -> Self;

    /// IEEE square root, correctly rounded to nearest-even.
    #[logic]
    fn sqrt_log(self) -> Self;

    /// Rounding to an integral value, ties to even. Exact.
    #[logic]
    fn round_ties_even_log(self) -> Self;

    /// IEEE fused multiply-add: `self · a + b`, rounded once to nearest-even.
    #[logic]
    fn mul_add_log(self, a: Self, b: Self) -> Self;
}

impl FloatLogic for f32 {
    #[logic]
    #[builtin("creusot.float.Float32.to_real")]
    fn to_real(self) -> Real {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.is_finite")]
    fn is_finite_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.is_nan")]
    fn is_nan_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.is_infinite")]
    fn is_infinite_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.is_zero")]
    fn is_zero_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.abs")]
    fn abs_log(self) -> Self {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.sqrt")]
    fn sqrt_log(self) -> Self {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.round_ties_even")]
    fn round_ties_even_log(self) -> Self {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float32.mul_add")]
    fn mul_add_log(self, _a: Self, _b: Self) -> Self {
        dead
    }
}

impl FloatLogic for f64 {
    #[logic]
    #[builtin("creusot.float.Float64.to_real")]
    fn to_real(self) -> Real {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.is_finite")]
    fn is_finite_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.is_nan")]
    fn is_nan_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.is_infinite")]
    fn is_infinite_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.is_zero")]
    fn is_zero_log(self) -> bool {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.abs")]
    fn abs_log(self) -> Self {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.sqrt")]
    fn sqrt_log(self) -> Self {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.round_ties_even")]
    fn round_ties_even_log(self) -> Self {
        dead
    }

    #[logic]
    #[builtin("creusot.float.Float64.mul_add")]
    fn mul_add_log(self, _a: Self, _b: Self) -> Self {
        dead
    }
}

/// `x` is finite and `x as f32` rounds to a finite `f32`: the precondition the
/// program-level `f64 as f32` cast asserts. Stated through the prelude's own
/// predicate, so a caller that establishes it discharges the cast's guard
/// syntactically instead of making the provers relate a hand-written bound
/// on `f32::MAX` to IEEE rounding.
#[logic]
#[builtin("creusot.float.FloatConvert.narrowing_in_range")]
pub fn narrows_to_f32(_x: f64) -> bool {
    dead
}

/// Scaling by a power of two is exact while the result stays in range: only
/// the exponent changes, also for a subnormal `x`. Bounded by `2^64`, below
/// both formats' `MAX` and the largest power of two the solvers know as a
/// number (`bv.Pow2int`). An IEEE 754 fact the solvers do not derive, so it
/// is assumed.
///
/// A ghost function rather than a lemma: a call instantiates the contract at
/// its arguments, whereas the solvers never instantiate a lemma about float
/// arithmetic, having no uninterpreted symbol to trigger on. Stated with
/// `2^k` rather than `scale.to_real()`, because a product of two float reals
/// stalls the solvers even where it follows by substitution.
#[trusted]
#[check(ghost)]
#[requires(x.is_finite_log() && scale.is_finite_log())]
#[requires(0 <= *k && scale.to_real() == Real::from_int(Int::pow2(*k)))]
#[requires(Real::from_int(-Int::pow2(64)) <= x.to_real() * Real::from_int(Int::pow2(*k)))]
#[requires(x.to_real() * Real::from_int(Int::pow2(*k)) <= Real::from_int(Int::pow2(64)))]
#[ensures((x * scale).is_finite_log())]
#[ensures((x * scale).to_real() == x.to_real() * Real::from_int(Int::pow2(*k)))]
pub fn exact_scaling<T: FloatLogic + MulLogic<Output = T>>(x: T, scale: T, k: Snapshot<Int>) {
    let _ = (x, scale, k);
}

// IEEE arithmetic in logic, rounded to nearest-even like the program-level
// operators: `a * b` in a contract is the product the code computes.

impl AddLogic for f32 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float32.add")]
    #[allow(unused_variables)]
    fn add_logic(self, other: Self) -> Self {
        dead
    }
}

impl SubLogic for f32 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float32.sub")]
    #[allow(unused_variables)]
    fn sub_logic(self, other: Self) -> Self {
        dead
    }
}

impl MulLogic for f32 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float32.mul")]
    #[allow(unused_variables)]
    fn mul_logic(self, other: Self) -> Self {
        dead
    }
}

impl DivLogic for f32 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float32.div")]
    #[allow(unused_variables)]
    fn div_logic(self, other: Self) -> Self {
        dead
    }
}

impl NegLogic for f32 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float32.neg")]
    fn neg_logic(self) -> Self {
        dead
    }
}

impl AddLogic for f64 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float64.add")]
    #[allow(unused_variables)]
    fn add_logic(self, other: Self) -> Self {
        dead
    }
}

impl SubLogic for f64 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float64.sub")]
    #[allow(unused_variables)]
    fn sub_logic(self, other: Self) -> Self {
        dead
    }
}

impl MulLogic for f64 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float64.mul")]
    #[allow(unused_variables)]
    fn mul_logic(self, other: Self) -> Self {
        dead
    }
}

impl DivLogic for f64 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float64.div")]
    #[allow(unused_variables)]
    fn div_logic(self, other: Self) -> Self {
        dead
    }
}

impl NegLogic for f64 {
    type Output = Self;

    #[logic]
    #[builtin("creusot.float.Float64.neg")]
    fn neg_logic(self) -> Self {
        dead
    }
}
