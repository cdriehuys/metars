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
    pub altimeter: u16,
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

        if let Some((raw_numerator, raw_denominator)) = raw_distance.split_once('/') {
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

    /// A vector of cloud layers in the reported order.
    Layers(Vec<CloudLayer>),
}

/// A single layer of clouds.
#[derive(Debug, PartialEq)]
pub struct CloudLayer {
    /// The type of cloud layer reported.
    pub kind: CloudKind,
    /// The height of the layer above the ground.
    pub agl: u16,
}

/// A type of cloud layer.
#[derive(Debug, PartialEq)]
pub enum CloudKind {
    Few,
    Scattered,
    Broken,
    Overcast,
}

impl FromStr for CloudKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BKN" => Ok(Self::Broken),
            "FEW" => Ok(Self::Few),
            "OVC" => Ok(Self::Overcast),
            "SCT" => Ok(Self::Scattered),
            _ => Err(()),
        }
    }
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

    #[test]
    fn cloud_kind_from_str() {
        let cases = vec![
            ("BKN", Some(CloudKind::Broken)),
            ("FEW", Some(CloudKind::Few)),
            ("OVC", Some(CloudKind::Overcast)),
            ("SCT", Some(CloudKind::Scattered)),
            ("UNKNOWN", None),
        ];
        for (name, expected) in cases {
            if let Some(want_value) = expected {
                assert_eq!(want_value, name.parse::<CloudKind>().expect("should parse"));
            } else {
                name.parse::<CloudKind>().expect_err("Should not be found.");
            }
        }
    }
}
