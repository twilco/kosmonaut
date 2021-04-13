use app_units::Au;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

/// A CSS float value.
pub type CSSFloat = f32;

/// A CSS integer value.
pub type CSSInteger = i32;

/// Number of app units per pixel
pub const AU_PER_PX: CSSFloat = 60.;
/// Number of app units per inch
pub const AU_PER_IN: CSSFloat = AU_PER_PX * 96.;
/// Number of app units per centimeter
pub const AU_PER_CM: CSSFloat = AU_PER_IN / 2.54;
/// Number of app units per millimeter
pub const AU_PER_MM: CSSFloat = AU_PER_IN / 25.4;
/// Number of app units per quarter
pub const AU_PER_Q: CSSFloat = AU_PER_MM / 4.;
/// Number of app units per point
pub const AU_PER_PT: CSSFloat = AU_PER_IN / 72.;
/// Number of app units per pica
pub const AU_PER_PC: CSSFloat = AU_PER_PT * 12.;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct CSSPixelLength(CSSFloat);

impl From<CSSPixelLength> for Au {
    #[inline]
    fn from(len: CSSPixelLength) -> Self {
        Au::from_f32_px(len.0)
    }
}

impl From<Au> for CSSPixelLength {
    #[inline]
    fn from(len: Au) -> Self {
        CSSPixelLength::new(len.to_f32_px())
    }
}

impl From<CSSFloat> for CSSPixelLength {
    fn from(val: f32) -> Self {
        CSSPixelLength::new(val)
    }
}

impl From<CSSPixelLength> for CSSFloat {
    fn from(val: CSSPixelLength) -> Self {
        val.0
    }
}

impl Add for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self::new(self.px() + other.px())
    }
}

impl Add<CSSFloat> for CSSPixelLength {
    type Output = CSSPixelLength;

    fn add(self, rhs: CSSFloat) -> Self::Output {
        (self.0 + rhs).into()
    }
}

impl Add<CSSPixelLength> for CSSFloat {
    type Output = CSSPixelLength;

    fn add(self, rhs: CSSPixelLength) -> Self::Output {
        (self + rhs.0).into()
    }
}

impl AddAssign for CSSPixelLength {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Div<CSSFloat> for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn div(self, other: CSSFloat) -> Self {
        Self::new(self.px() / other)
    }
}

impl Mul<CSSFloat> for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn mul(self, other: CSSFloat) -> Self {
        Self::new(self.px() * other)
    }
}

impl MulAssign<CSSFloat> for CSSPixelLength {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 = self.0 * rhs;
    }
}

impl Neg for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        CSSPixelLength::new(-self.0)
    }
}

impl Sub for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self::new(self.px() - other.px())
    }
}

impl Sub<CSSFloat> for CSSPixelLength {
    type Output = Self;

    #[inline]
    fn sub(self, other: CSSFloat) -> Self {
        Self::new(self.px() - other)
    }
}

impl Sub<CSSPixelLength> for CSSFloat {
    type Output = CSSPixelLength;

    fn sub(self, rhs: CSSPixelLength) -> Self::Output {
        CSSPixelLength::new(self - rhs.px())
    }
}

impl PartialOrd<CSSFloat> for CSSPixelLength {
    fn partial_cmp(&self, other: &CSSFloat) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<CSSFloat> for CSSPixelLength {
    fn eq(&self, other: &CSSFloat) -> bool {
        self.0 == *other
    }
}

impl CSSPixelLength {
    /// Return a new CSSPixelLength.
    #[inline]
    pub fn new(px: CSSFloat) -> Self {
        CSSPixelLength(px)
    }

    /// Return the containing pixel value.
    #[inline]
    pub fn px(self) -> CSSFloat {
        self.0
    }

    /// Return the length with app_unit i32 type.
    #[inline]
    pub fn to_i32_au(self) -> i32 {
        Au::from(self).0
    }

    /// Return the absolute value of this length.
    #[inline]
    pub fn abs(self) -> Self {
        CSSPixelLength::new(self.0.abs())
    }

    /// Return the clamped value of this length.
    #[inline]
    pub fn clamp_to_non_negative(self) -> Self {
        CSSPixelLength::new(self.0.max(0.))
    }

    /// Returns the minimum between `self` and `other`.
    #[inline]
    pub fn min(self, other: Self) -> Self {
        CSSPixelLength::new(self.0.min(other.0))
    }

    /// Returns the maximum between `self` and `other`.
    #[inline]
    pub fn max(self, other: Self) -> Self {
        CSSPixelLength::new(self.0.max(other.0))
    }

    /// Sets `self` to the maximum between `self` and `other`.
    pub fn max_assign(&mut self, other: Self) {
        *self = self.max(other);
    }
}
