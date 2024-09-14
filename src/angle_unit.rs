#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnit {
    Degrees,
    Hours,
}

impl AngleUnit {
    #[must_use]
    pub fn units_in_rotation(&self) -> u32 {
        match self {
            Self::Degrees => 360,
            Self::Hours => 24,
        }
    }

    #[must_use]
    pub fn parts_in_rotation(&self, subdivision: AngleUnitSubdivision) -> u32 {
        subdivision.parts_in_unit() * self.units_in_rotation()
    }

    #[must_use]
    pub fn subdivision_unit_symbols(&self) -> [char; 3] {
        match self {
            Self::Degrees => ['°', '′', '″'],
            Self::Hours => ['ʰ', 'ᵐ', 'ˢ'],
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnitSubdivision {
    Wholes,
    Minutes,
    Seconds,
}

impl AngleUnitSubdivision {
    #[must_use]
    pub fn parts_in_unit(&self) -> u32 {
        match self {
            Self::Wholes => 1,
            Self::Minutes => 60,
            Self::Seconds => 3600,
        }
    }
}
