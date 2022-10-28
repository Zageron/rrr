use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// An example of wrapping a standard type for better readability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceptorPosition(pub u32);

impl Add for ReceptorPosition {
    type Output = ReceptorPosition;

    fn add(self, rhs: Self) -> Self::Output {
        ReceptorPosition(self.0 + rhs.0)
    }
}

impl Sub for ReceptorPosition {
    type Output = ReceptorPosition;

    fn sub(self, rhs: Self) -> Self::Output {
        ReceptorPosition(self.0 - rhs.0)
    }
}

impl Add<ReceptorPosition> for u32 {
    type Output = ReceptorPosition;

    fn add(self, rhs: ReceptorPosition) -> Self::Output {
        ReceptorPosition(self + rhs.0)
    }
}

impl Sub<ReceptorPosition> for u32 {
    type Output = ReceptorPosition;

    fn sub(self, rhs: ReceptorPosition) -> Self::Output {
        ReceptorPosition(self - rhs.0)
    }
}

impl Add<u32> for ReceptorPosition {
    type Output = ReceptorPosition;

    fn add(self, rhs: u32) -> Self::Output {
        ReceptorPosition(self.0 + rhs)
    }
}

impl Sub<u32> for ReceptorPosition {
    type Output = ReceptorPosition;

    fn sub(self, rhs: u32) -> Self::Output {
        ReceptorPosition(self.0 - rhs)
    }
}

impl Default for ReceptorPosition {
    fn default() -> Self {
        Self(64)
    }
}
