use crate::{AngleUnit, AngleUnitSubdivision};

pub trait AngleFrom: Sized {
    fn from_degrees(degrees: f64) -> Self;

    #[must_use]
    fn from_hours(hours: f64) -> Self {
        Self::from_degrees(15.0 * hours)
    }

    #[must_use]
    fn from_unit(value: f64, unit: AngleUnit) -> Self {
        match unit {
            AngleUnit::Degrees => Self::from_degrees(value),
            AngleUnit::Hours => Self::from_hours(value),
        }
    }

    #[must_use]
    fn from_unit_subdivision(
        value: f64,
        unit: AngleUnit,
        subdivision: AngleUnitSubdivision,
    ) -> Self {
        Self::from_unit(
            value / f64::from(subdivision.parts_in_unit()),
            unit,
        )
    }
}

pub trait AngleTryFrom: Sized {
    type Error;

    fn try_from_degrees(degrees: f64) -> Result<Self, Self::Error>;

    fn try_from_hours(hours: f64) -> Result<Self, Self::Error> {
        Self::try_from_degrees(15.0 * hours)
    }

    fn try_from_unit(value: f64, unit: AngleUnit) -> Result<Self, Self::Error> {
        match unit {
            AngleUnit::Degrees => Self::try_from_degrees(value),
            AngleUnit::Hours => Self::try_from_hours(value),
        }
    }

    fn try_from_unit_subdivision(
        value: f64,
        unit: AngleUnit,
        subdivision: AngleUnitSubdivision,
    ) -> Result<Self, Self::Error> {
        Self::try_from_unit(
            value / f64::from(subdivision.parts_in_unit()),
            unit,
        )
    }
}
