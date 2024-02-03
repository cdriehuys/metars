use std::str::FromStr;

use parser::Rule;
use pest::Parser;

mod parser;

#[derive(Debug)]
pub enum ParseError {
    MissingElement(String),
    MalformedInput(pest::error::Error<Rule>),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Self::MalformedInput(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct Metar {
    pub station: String,
    pub observation_time: String,
}

impl FromStr for Metar {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = parser::MetarParser::parse(Rule::METAR, s)?.next().unwrap();

        let mut station = None;
        let mut observation_time = None;
        for record in parsed.into_inner() {
            match record.as_rule() {
                Rule::station => {
                    station = Some(record.as_str().to_owned());
                }
                Rule::observation_time => {
                    observation_time = Some(record.as_str().to_owned());
                }
                _ => unreachable!(),
            }
        }

        Ok(Metar {
            station: station
                .ok_or_else(|| ParseError::MissingElement("Station name".to_owned()))?,
            observation_time: observation_time
                .ok_or_else(|| ParseError::MissingElement("Observation time".to_owned()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_metar() {
        let raw = "KTTA 031530Z AUTO 04008KT 10SM CLR 07/M02";
        let expected = Metar {
            station: "KTTA".to_owned(),
            observation_time: "031530Z".to_owned(),
        };

        let received: Metar = raw.parse().expect("should be parseable");

        assert_eq!(expected, received);
    }
}
