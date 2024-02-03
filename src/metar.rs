use std::str::FromStr;

use crate::{parser::parse_metar, ParseError};

#[derive(Debug, PartialEq)]
pub struct Metar {
    pub station: String,
    pub observation_time: String,
    pub automated_report: bool,
    pub wind: String,
}

impl FromStr for Metar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_metar(s)
    }
}
