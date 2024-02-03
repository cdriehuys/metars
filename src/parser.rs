use pest::Parser;
use pest_derive::Parser;

use crate::{Metar, Wind};

#[derive(Parser)]
#[grammar = "metar.pest"]
pub struct MetarParser;

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

pub fn parse_metar(metar: &str) -> Result<Metar, ParseError> {
    let parsed = MetarParser::parse(Rule::METAR, metar)?.next().unwrap();

    let mut station = None;
    let mut observation_time = None;
    let mut automated_report = false;
    let mut wind = None;

    for pair in parsed.into_inner() {
        match pair.as_rule() {
            Rule::station => {
                station = Some(pair.as_str().to_owned());
            }
            Rule::observation_time => {
                observation_time = Some(pair.as_str().to_owned());
            }
            Rule::auto_kw => {
                automated_report = true;
            }
            Rule::wind => {
                // Wind is defined as a direction followed by a speed.
                let mut pairs = pair.into_inner();

                let raw_direction = pairs.next().unwrap().as_str();
                let raw_speed = pairs.next().unwrap().as_str();

                wind = Some(Wind {
                    direction: raw_direction.parse().unwrap(),
                    speed: raw_speed.parse().unwrap(),
                });
            }
            _ => unreachable!(),
        }
    }

    Ok(Metar {
        station: station.ok_or_else(|| ParseError::MissingElement("Station name".to_owned()))?,
        observation_time: observation_time
            .ok_or_else(|| ParseError::MissingElement("Observation time".to_owned()))?,
        automated_report,
        wind: wind.ok_or_else(|| ParseError::MissingElement("Wind".to_owned()))?,
    })
}
