use std::{num::ParseFloatError, str::FromStr};

use crate::{parser::parse_metar, ParseError};

#[derive(Debug, PartialEq)]
pub struct Metar {
    pub station: String,
    pub observation_time: String,
    pub automated_report: bool,
    pub wind: Wind,
    pub visibility: Visibility,
    pub clouds: Clouds,
    pub temp: i8,
    pub dewpoint: i8,
}

impl FromStr for Metar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_metar(s)
    }
}

/// Wind speed and direction.
#[derive(Debug, PartialEq)]
pub struct Wind {
    /// True wind direction.
    pub direction: u16,

    /// Wind speed in knots.
    pub speed: u8,
}

#[derive(Debug, PartialEq)]
pub enum Visibility {
    /// Visibility in statute miles
    SM(f32),
}

impl FromStr for Visibility {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // All distance units are 2 characters.
        let raw_distance = &s[..s.len() - 2];

        if let Some((raw_numerator, raw_denominator)) = raw_distance.split_once("/") {
            let numerator: f32 = raw_numerator.parse()?;
            let denominator: f32 = raw_denominator.parse()?;

            Ok(Self::SM(numerator / denominator))
        } else {
            Ok(Self::SM(raw_distance.parse()?))
        }
    }
}

/// Reported cloud layers.
#[derive(Debug, PartialEq)]
pub enum Clouds {
    /// No reported clouds.
    Clear,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn visibility_single_digit_int() {
        let visibility = "3SM";
        let parsed: Visibility = visibility.parse().expect("parseable");

        assert_eq!(Visibility::SM(3.0), parsed);
    }

    #[test]
    fn visibility_double_digit_int() {
        let visibility = "10SM";
        let parsed: Visibility = visibility.parse().expect("parseable");

        assert_eq!(Visibility::SM(10.0), parsed);
    }

    #[test]
    fn visibility_fractional() {
        let visibility = "1/2SM";
        let parsed: Visibility = visibility.parse().expect("parseable");

        assert_eq!(Visibility::SM(0.5), parsed);
    }
}
