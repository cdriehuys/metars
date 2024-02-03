use std::str::FromStr;

use crate::{parser::parse_metar, ParseError};

#[derive(Debug, PartialEq)]
pub struct Metar {
    pub station: String,
    pub observation_time: String,
    pub automated_report: bool,
    pub wind: Wind,
    pub visibility: Visibility,
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
    SM(u8),
}
