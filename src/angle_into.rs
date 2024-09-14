use crate::{Angle, AngleUnit, AngleUnitSubdivision};

#[derive(Debug, Clone, Copy)]
pub enum AngleBound {
    LowerInclusive(Angle),
    UpperInclusive(Angle),
}

pub trait AngleInto {
    fn to_degrees(&self) -> f64;

    fn to_hours(&self) -> f64 {
        self.to_degrees() / 15.0
    }

    fn to_unit(&self, unit: AngleUnit) -> f64 {
        match unit {
            AngleUnit::Degrees => self.to_degrees(),
            AngleUnit::Hours => self.to_hours(),
        }
    }

    fn to_unit_subdivision(
        &self,
        unit: AngleUnit,
        subdivision: AngleUnitSubdivision,
    ) -> f64 {
        f64::from(subdivision.parts_in_unit()) * self.to_unit(unit)
    }

    fn to_unit_subdivision_normalized(
        &self,
        unit: AngleUnit,
        subdivision: AngleUnitSubdivision,
        bound: AngleBound,
    ) -> f64 {
        let angle = self.to_unit_subdivision(unit, subdivision);
        let rotation = f64::from(unit.parts_in_rotation(subdivision));

        match bound {
            AngleBound::LowerInclusive(bound) => {
                let lower_bound = bound.to_unit_subdivision(unit, subdivision);

                let difference = angle - lower_bound;
                let difference_mod =
                    (difference % rotation + rotation) % rotation;
                lower_bound + difference_mod
            }
            AngleBound::UpperInclusive(bound) => {
                let upper_bound = bound.to_unit_subdivision(unit, subdivision);

                let difference = upper_bound - angle;
                let difference_mod =
                    (difference % rotation + rotation) % rotation;
                upper_bound - difference_mod
            }
        }
    }

    fn to_degrees_nonnegative(&self) -> f64 {
        self.to_unit_subdivision_normalized(
            AngleUnit::Degrees,
            AngleUnitSubdivision::Wholes,
            AngleBound::LowerInclusive(Angle::ZERO),
        )
    }

    fn to_degrees_symmetric(&self) -> f64 {
        self.to_unit_subdivision_normalized(
            AngleUnit::Degrees,
            AngleUnitSubdivision::Wholes,
            AngleBound::UpperInclusive(Angle::HALF_ROTATION),
        )
    }

    fn to_hours_nonnegative(&self) -> f64 {
        self.to_unit_subdivision_normalized(
            AngleUnit::Hours,
            AngleUnitSubdivision::Wholes,
            AngleBound::LowerInclusive(Angle::ZERO),
        )
    }

    fn to_hours_symmetric(&self) -> f64 {
        self.to_unit_subdivision_normalized(
            AngleUnit::Hours,
            AngleUnitSubdivision::Wholes,
            AngleBound::UpperInclusive(Angle::HALF_ROTATION),
        )
    }
}
