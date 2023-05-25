pub type Uf64 = UncertainFloat;
pub type RealFunc = dyn Fn(f64) -> f64;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UncertainFloat {
    pub value: f64,
    pub uncertainty: f64,
}

use num_traits::{Num, NumCast, FromPrimitive};
use std::ops::{Add, Sub, Mul, Div, Rem};

impl UncertainFloat {
    pub fn new(value: f64, uncertainty: f64) -> Self {
        UncertainFloat {
            value,
            uncertainty,
        }
    }
    
    pub fn apply(&self, f: &RealFunc) -> Self {
        self.apply_with_central_difference_approx_order_four(f)
    }

    /// Apply function with Finite Difference Approximation of order 1
    pub fn apply_with_finite_difference_approx(&self, f: &RealFunc) -> Self {
        let h = 1e-9; // finite difference approximation
        let value = f(self.value);
        let derivative = (f(self.value + h) - f(self.value - h)) / (2.0 * h);
        let uncertainty = self.uncertainty * derivative.abs();
        Self::new(value, uncertainty)
    }

    /// Apply function with central difference approximation of order 4
    pub fn apply_with_central_difference_approx_order_four(&self, f: &RealFunc) -> Self {
        let h = 1e-6;  // A small number for finite difference approximation
        let value = f(self.value);
        
        let derivative = (-f(self.value + 2.0 * h) + 8.0 * f(self.value + h) - 8.0 * f(self.value - h) + f(self.value - 2.0 * h)) / (12.0 * h);
        
        let uncertainty = self.uncertainty * derivative.abs();
        Self::new(value, uncertainty)
    }
}

impl std::ops::Add for UncertainFloat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let value = self.value + other.value;
        let uncertainty = (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt();
        Self::new(value, uncertainty)
    }
}

impl std::ops::Sub for UncertainFloat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let value = self.value - other.value;
        let uncertainty = (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt();
        Self::new(value, uncertainty)
    }
}

impl std::ops::Mul for UncertainFloat {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let value = self.value * other.value;
        let uncertainty = ((self.uncertainty / self.value).powi(2) + (other.uncertainty / other.value).powi(2)).sqrt() * value;
        Self::new(value, uncertainty)
    }
}

impl std::ops::Div for UncertainFloat {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let value = self.value / other.value;
        let uncertainty = ((self.uncertainty / self.value).powi(2) + (other.uncertainty / other.value).powi(2)).sqrt() * value;
        Self::new(value, uncertainty)
    }
}

impl std::ops::Neg for UncertainFloat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        UncertainFloat {
            value: -self.value,
            uncertainty: self.uncertainty,
        }
    }
}

/// Dummy implementation for REM
/// Had to be implemented
/// TODO: does this create any problems?
impl std::ops::Rem for UncertainFloat {
    type Output = Self;

    fn rem(self, _rhs: Self) -> Self::Output {
        UncertainFloat {
            value: 0.0,
            uncertainty: 0.0,
        }
    }
}

impl num_traits::NumCast for UncertainFloat {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(UncertainFloat {
            value: n.to_f64()?,
            uncertainty: 0.0,
        })
    }
}

impl num_traits::ToPrimitive for UncertainFloat {
    fn to_i64(&self) -> Option<i64> {
        Some(self.value as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        Some(self.value as u64)
    }

    fn to_f64(&self) -> Option<f64> {
        Some(self.value)
    }
    // implement other methods as required
}

impl ndarray::ScalarOperand for UncertainFloat {}

impl num_traits::Num for UncertainFloat {
    type FromStrRadixErr = num_traits::ParseFloatError;//std::num::ParseFloatError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let val = f64::from_str_radix(str, radix)?;
        Ok(UncertainFloat {
            value: val,
            uncertainty: 0.0,
        })
    }
}

impl num_traits::Zero for UncertainFloat {
    fn zero() -> Self {
        UncertainFloat {
            value: 0.0,
            uncertainty: 0.0,
        }
    }

    fn is_zero(&self) -> bool {
        self.value == 0.0
    }
    
}

impl num_traits::One for UncertainFloat {
    fn one() -> Self {
        UncertainFloat {
            value: 1.0,
            uncertainty: 0.0,
        }
    }
}

const LN_2: f64 = 0.6931471805599453;
const LN_10: f64 = 2.302585092994046;

impl num_traits::Float for UncertainFloat {
    fn nan() -> Self {
        UncertainFloat::new(f64::NAN, f64::NAN)
    }

    fn infinity() -> Self {
        UncertainFloat::new(f64::INFINITY, 0.0)
    }

    fn neg_infinity() -> Self {
        UncertainFloat::new(f64::NEG_INFINITY, 0.0)
    }

    fn neg_zero() -> Self {
        UncertainFloat::new(-0.0, 0.0)
    }

    fn min_value() -> Self {
        UncertainFloat::new(f64::MIN, 0.0)
    }

    fn min_positive_value() -> Self {
        UncertainFloat::new(f64::MIN_POSITIVE, 0.0)
    }

    fn max_value() -> Self {
        UncertainFloat::new(f64::MAX, 0.0)
    }

    fn is_nan(self) -> bool {
        self.value.is_nan()
    }

    fn is_infinite(self) -> bool {
        self.value.is_infinite()
    }

    fn is_finite(self) -> bool {
        self.value.is_finite()
    }

    fn is_normal(self) -> bool {
        self.value.is_normal()
    }

    fn classify(self) -> std::num::FpCategory {
        self.value.classify()
    }

    fn floor(self) -> Self {
        UncertainFloat::new(self.value.floor(), 0.0)
    }

    fn ceil(self) -> Self {
        UncertainFloat::new(self.value.ceil(), 0.0)
    }

    fn round(self) -> Self {
        UncertainFloat::new(self.value.round(), 0.0)
    }

    fn trunc(self) -> Self {
        UncertainFloat::new(self.value.trunc(), 0.0)
    }

    fn fract(self) -> Self {
        UncertainFloat::new(self.value.fract(), 0.0)
    }

    fn abs(self) -> Self {
        UncertainFloat::new(self.value.abs(), self.uncertainty)
    }

    fn signum(self) -> Self {
        UncertainFloat::new(self.value.signum(), 0.0)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        UncertainFloat::new(self.value.mul_add(a.value, b.value), 0.0)
    }

    fn recip(self) -> Self {
        UncertainFloat::new(self.value.recip(), self.uncertainty / self.value.powi(2))
    }

    fn powi(self, n: i32) -> Self {
        UncertainFloat::new(self.value.powi(n), self.uncertainty * (n as f64) * self.value.powi(n - 1))
    }

    fn powf(self, n: Self) -> Self {
        UncertainFloat::new(self.value.powf(n.value), self.uncertainty * n.value * self.value.powf(n.value - 1.0))
    }

    fn sqrt(self) -> Self {
        UncertainFloat::new(self.value.sqrt(), self.uncertainty / (2.0 * self.value.sqrt()))
    }

    fn exp(self) -> Self {
        UncertainFloat::new(self.value.exp(), self.uncertainty * self.value.exp())
    }

    fn exp2(self) -> Self {
        UncertainFloat::new(self.value.exp2(), self.uncertainty * self.value.exp2() * LN_2)
    }

    fn ln(self) -> Self {
        UncertainFloat::new(self.value.ln(), self.uncertainty / self.value)
    }

    fn log(self, base: Self) -> Self {
        UncertainFloat::new(self.value.log(base.value), self.uncertainty / (self.value * base.value.ln()))
    }

    fn log2(self) -> Self {
        UncertainFloat::new(self.value.log2(), self.uncertainty / (self.value * LN_2))
    }

    fn log10(self) -> Self {
        UncertainFloat::new(self.value.log10(), self.uncertainty / (self.value * LN_10))
    }

    fn max(self, other: Self) -> Self {
        if self.value > other.value {
            self
        } else if self.value < other.value {
            other
        } else {
            UncertainFloat::new(self.value, (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt())
        }
    }

    fn min(self, other: Self) -> Self {
        if self.value < other.value {
            self
        } else if self.value > other.value {
            other
        } else {
            UncertainFloat::new(self.value, (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt())
        }
    }

    fn abs_sub(self, other: Self) -> Self {
        if self.value > other.value {
            UncertainFloat::new(self.value - other.value, self.uncertainty + other.uncertainty)
        } else {
            UncertainFloat::new(other.value - self.value, self.uncertainty + other.uncertainty)
        }
    }

    fn cbrt(self) -> Self {
        UncertainFloat::new(self.value.cbrt(), self.uncertainty / (3.0 * self.value.powi(2)))
    }

    fn hypot(self, other: Self) -> Self {
        UncertainFloat::new(self.value.hypot(other.value), (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt())
    }

    fn sin(self) -> Self {
        UncertainFloat::new(self.value.sin(), self.uncertainty * self.value.cos())
    }

    fn cos(self) -> Self {
        UncertainFloat::new(self.value.cos(), self.uncertainty * self.value.sin())
    }

    fn tan(self) -> Self {
        UncertainFloat::new(self.value.tan(), self.uncertainty * (1.0 + self.value.tan().powi(2)))
    }

    fn asin(self) -> Self {
        UncertainFloat::new(self.value.asin(), self.uncertainty / (1.0 - self.value.powi(2)).sqrt())
    }

    fn acos(self) -> Self {
        UncertainFloat::new(self.value.acos(), self.uncertainty / (1.0 - self.value.powi(2)).sqrt())
    }

    fn atan(self) -> Self {
        UncertainFloat::new(self.value.atan(), self.uncertainty / (1.0 + self.value.powi(2)))
    }

    fn atan2(self, other: Self) -> Self {
        UncertainFloat::new(self.value.atan2(other.value), (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt() / (self.value.powi(2) + other.value.powi(2)))
    }

    fn sin_cos(self) -> (Self, Self) {
        let (sin, cos) = self.value.sin_cos();
        let sin_uncertainty = self.uncertainty * cos;
        let cos_uncertainty = self.uncertainty * sin;
        (UncertainFloat::new(sin, sin_uncertainty), UncertainFloat::new(cos, cos_uncertainty))
    }

    fn exp_m1(self) -> Self {
        UncertainFloat::new(self.value.exp_m1(), self.uncertainty * self.value.exp())
    }

    fn ln_1p(self) -> Self {
        UncertainFloat::new(self.value.ln_1p(), self.uncertainty / (1.0 + self.value))
    }

    fn sinh(self) -> Self {
        UncertainFloat::new(self.value.sinh(), self.uncertainty * self.value.cosh())
    }

    fn cosh(self) -> Self {
        UncertainFloat::new(self.value.cosh(), self.uncertainty * self.value.sinh())
    }

    fn tanh(self) -> Self {
        UncertainFloat::new(self.value.tanh(), self.uncertainty * (1.0 - self.value.tanh().powi(2)))
    }

    fn asinh(self) -> Self {
        UncertainFloat::new(self.value.asinh(), self.uncertainty / (1.0 + self.value.powi(2)).sqrt())
    }

    fn acosh(self) -> Self {
        UncertainFloat::new(self.value.acosh(), self.uncertainty / (self.value.powi(2) - 1.0).sqrt())
    }

    fn atanh(self) -> Self {
        UncertainFloat::new(self.value.atanh(), self.uncertainty / (1.0 - self.value.powi(2)))
    }

    /*fn rem(self, other: Self) -> Self {
        let (q, _) = self.value.div_rem(&other.value);
        UncertainFloat::new(self.value - q * other.value, self.uncertainty)
    }*/
    fn is_sign_positive(self) -> bool {
        self.value.is_sign_positive()
    }

    fn is_sign_negative(self) -> bool {
        self.value.is_sign_negative()
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.value.integer_decode()
    }
}

