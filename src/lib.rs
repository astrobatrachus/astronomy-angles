#![warn(clippy::pedantic)]

use std::ops::{Add, Mul, Sub};

use thiserror::Error;

mod angle_from;
mod angle_into;
mod angle_unit;
mod format;

pub use angle_from::{AngleFrom, AngleTryFrom};
pub use angle_into::{AngleBound, AngleInto};
pub use angle_unit::{AngleUnit, AngleUnitSubdivision};
pub use format::{AngleFormat, AngleRange, AngleUnitPrecision};

#[derive(Debug, Clone, Copy)]
pub struct Angle(f64);

impl Angle {
    pub const ZERO: Angle = Angle(0.0);

    pub const HALF_ROTATION: Angle = Angle(180.0);
}

impl AngleFrom for Angle {
    fn from_degrees(degrees: f64) -> Self {
        Self(degrees)
    }
}

impl AngleInto for Angle {
    fn to_degrees(&self) -> f64 {
        self.0
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Self::from_degrees(self.to_degrees() + rhs.to_degrees())
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Self::Output {
        Self::from_degrees(self.to_degrees() - rhs.to_degrees())
    }
}

impl Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Self::Output {
        Angle::from_degrees(self * rhs.to_degrees())
    }
}

impl From<AcuteAngle> for Angle {
    fn from(value: AcuteAngle) -> Self {
        Self::from_degrees(value.to_degrees())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AcuteAngle(f64);

#[derive(Debug, Error)]
#[error("value outside the allowed range [-90°, 90°]")]
pub struct AcuteAngleError;

impl AngleTryFrom for AcuteAngle {
    type Error = AcuteAngleError;

    fn try_from_degrees(degrees: f64) -> Result<Self, Self::Error> {
        if (-90.0..=90.0).contains(&degrees) {
            Ok(Self(degrees))
        } else {
            Err(AcuteAngleError)
        }
    }
}

impl AngleInto for AcuteAngle {
    fn to_degrees(&self) -> f64 {
        self.0
    }
}

impl TryFrom<Angle> for AcuteAngle {
    type Error = AcuteAngleError;

    fn try_from(value: Angle) -> Result<Self, Self::Error> {
        Self::try_from_degrees(value.to_degrees())
    }
}
