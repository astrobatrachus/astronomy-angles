use rust_decimal::{Decimal, RoundingStrategy};

use crate::{
    Angle, AngleBound, AngleFrom, AngleInto, AngleUnit, AngleUnitSubdivision,
};

#[allow(clippy::module_name_repetitions)]
pub trait AngleFormat {
    fn format_angle(
        &self,
        unit_precision: AngleUnitPrecision,
        range: AngleRange,
    ) -> String;
}

impl<A> AngleFormat for A
where
    A: AngleInto,
{
    fn format_angle(
        &self,
        unit_precision: AngleUnitPrecision,
        range: AngleRange,
    ) -> String {
        let (unit, subdivision, precision) = unit_precision.into();
        format_angle_inner(self, unit, subdivision, precision, range)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AngleUnitPrecision {
    Degrees(u32),
    DegreeMinutes(u32),
    DegreeSeconds(u32),
    Hours(u32),
    HourMinutes(u32),
    HourSeconds(u32),
}

impl From<AngleUnitPrecision> for (AngleUnit, AngleUnitSubdivision, u32) {
    fn from(value: AngleUnitPrecision) -> Self {
        match value {
            AngleUnitPrecision::Degrees(precision) => (
                AngleUnit::Degrees,
                AngleUnitSubdivision::Wholes,
                precision,
            ),
            AngleUnitPrecision::DegreeMinutes(precision) => (
                AngleUnit::Degrees,
                AngleUnitSubdivision::Minutes,
                precision,
            ),
            AngleUnitPrecision::DegreeSeconds(precision) => (
                AngleUnit::Degrees,
                AngleUnitSubdivision::Seconds,
                precision,
            ),
            AngleUnitPrecision::Hours(precision) => (
                AngleUnit::Hours,
                AngleUnitSubdivision::Wholes,
                precision,
            ),
            AngleUnitPrecision::HourMinutes(precision) => (
                AngleUnit::Hours,
                AngleUnitSubdivision::Minutes,
                precision,
            ),
            AngleUnitPrecision::HourSeconds(precision) => (
                AngleUnit::Hours,
                AngleUnitSubdivision::Seconds,
                precision,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AngleRange {
    /// The range [0°, 360°)
    NonNegative,
    /// The range (-180°, 180°]
    Symmetric,
}

impl From<AngleRange> for AngleBound {
    fn from(value: AngleRange) -> Self {
        match value {
            AngleRange::NonNegative => Self::LowerInclusive(Angle::ZERO),
            AngleRange::Symmetric => Self::UpperInclusive(Angle::HALF_ROTATION),
        }
    }
}

fn format_angle_inner(
    angle: &impl AngleInto,
    unit: AngleUnit,
    subdivision: AngleUnitSubdivision,
    precision: u32,
    range: AngleRange,
) -> String {
    // Normalization must be done before rounding to ensure correct rounding
    let angle_float = Angle::from_degrees(angle.to_degrees())
        .to_unit_subdivision_normalized(unit, subdivision, range.into());

    // The angle is normalized and hence always small
    let angle_rounded = Decimal::try_from(angle_float)
        .unwrap()
        .round_dp_with_strategy(
            precision,
            RoundingStrategy::MidpointAwayFromZero,
        );

    let rotation = Decimal::from(unit.parts_in_rotation(subdivision));

    let angle = match range {
        AngleRange::NonNegative => {
            (angle_rounded % rotation + rotation) % rotation
        }
        AngleRange::Symmetric => {
            let upper_bound = rotation / Decimal::TWO;

            let difference = upper_bound - angle_rounded;
            let difference_mod = (difference % rotation + rotation) % rotation;
            upper_bound - difference_mod
        }
    };

    let decimal_60 = Decimal::from(60);
    let decimal_3600 = Decimal::from(3600);

    let [w_sym, m_sym, s_sym] = unit.subdivision_unit_symbols();
    let prec = usize::try_from(precision).unwrap_or(usize::MAX);
    let sub_width = 2 + if prec > 0 { prec + 1 } else { 0 };

    match subdivision {
        AngleUnitSubdivision::Wholes => match range {
            AngleRange::NonNegative => format!("{angle:.prec$}{w_sym}"),
            AngleRange::Symmetric => format!("{angle:+.prec$}{w_sym}"),
        },

        AngleUnitSubdivision::Minutes => {
            let wholes = angle / decimal_60;
            let minutes = (angle % decimal_60).abs();

            match range {
                AngleRange::NonNegative => {
                    format!(
                        "{wholes:.0}{w_sym}\
                        {minutes:0sub_width$.prec$}{m_sym}"
                    )
                }
                AngleRange::Symmetric => {
                    format!(
                        "{wholes:+.0}{w_sym}\
                        {minutes:0sub_width$.prec$}{m_sym}"
                    )
                }
            }
        }

        AngleUnitSubdivision::Seconds => {
            let wholes = angle / decimal_3600;
            let minutes = (angle.abs() / decimal_60) % decimal_60;
            let seconds = angle.abs() % decimal_60;

            match range {
                AngleRange::NonNegative => {
                    format!(
                        "{wholes:.0}{w_sym}\
                        {minutes:02.0}{m_sym}\
                        {seconds:0sub_width$.prec$}{s_sym}"
                    )
                }
                AngleRange::Symmetric => {
                    format!(
                        "{wholes:+.0}{w_sym}\
                        {minutes:02.0}{m_sym}\
                        {seconds:0sub_width$.prec$}{s_sym}"
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Angle, AngleFormat, AngleFrom};

    use super::AngleRange;

    #[test]
    fn degrees() {
        assert_eq!(
            Angle::from_degrees(26.245).format_angle(
                crate::AngleUnitPrecision::Degrees(2),
                AngleRange::NonNegative
            ),
            "26.25°"
        );

        assert_eq!(
            Angle::from_degrees(270.65).format_angle(
                crate::AngleUnitPrecision::Degrees(1),
                AngleRange::Symmetric
            ),
            "-89.4°"
        );
    }

    #[test]
    fn degree_minutes() {
        assert_eq!(
            Angle::from_degrees(638.1523).format_angle(
                crate::AngleUnitPrecision::DegreeMinutes(0),
                AngleRange::NonNegative
            ),
            "278°09′"
        );

        assert_eq!(
            Angle::from_degrees(638.1523).format_angle(
                crate::AngleUnitPrecision::DegreeMinutes(2),
                AngleRange::Symmetric
            ),
            "-81°50.86′"
        );
    }

    #[test]
    fn degree_seconds() {
        assert_eq!(
            Angle::from_degrees(-23.085_925).format_angle(
                crate::AngleUnitPrecision::DegreeSeconds(2),
                AngleRange::NonNegative
            ),
            "336°54′50.67″"
        );

        assert_eq!(
            Angle::from_degrees(-23.085_925).format_angle(
                crate::AngleUnitPrecision::DegreeSeconds(2),
                AngleRange::Symmetric
            ),
            "-23°05′09.33″"
        );
    }

    #[test]
    fn hours() {
        assert_eq!(
            Angle::from_hours(31.734_579).format_angle(
                crate::AngleUnitPrecision::Hours(3),
                AngleRange::NonNegative
            ),
            "7.735ʰ"
        );

        assert_eq!(
            Angle::from_hours(31.734_579).format_angle(
                crate::AngleUnitPrecision::Hours(3),
                AngleRange::Symmetric
            ),
            "+7.735ʰ"
        );
    }

    #[test]
    fn hour_minutes() {
        assert_eq!(
            Angle::from_hours(22.276_198).format_angle(
                crate::AngleUnitPrecision::HourMinutes(3),
                AngleRange::NonNegative
            ),
            "22ʰ16.572ᵐ"
        );

        assert_eq!(
            Angle::from_hours(22.276_198).format_angle(
                crate::AngleUnitPrecision::HourMinutes(3),
                AngleRange::Symmetric
            ),
            "-1ʰ43.428ᵐ"
        );
    }

    #[test]
    fn hour_seconds() {
        assert_eq!(
            Angle::from_hours(21.685_442).format_angle(
                crate::AngleUnitPrecision::HourSeconds(0),
                AngleRange::NonNegative
            ),
            "21ʰ41ᵐ08ˢ"
        );

        assert_eq!(
            Angle::from_hours(21.685_442).format_angle(
                crate::AngleUnitPrecision::HourSeconds(2),
                AngleRange::Symmetric
            ),
            "-2ʰ18ᵐ52.41ˢ"
        );
    }

    #[test]
    fn rounding() {
        assert_eq!(
            Angle::from_degrees(359.999).format_angle(
                crate::AngleUnitPrecision::Degrees(2),
                AngleRange::NonNegative
            ),
            "0.00°"
        );

        assert_eq!(
            Angle::from_degrees(-179.5).format_angle(
                crate::AngleUnitPrecision::Degrees(0),
                AngleRange::Symmetric
            ),
            "+180°"
        );
    }
}
