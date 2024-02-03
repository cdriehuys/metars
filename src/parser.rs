use std::{error::Error, num::ParseFloatError};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{metar, Metar};

#[derive(Parser)]
#[grammar = "metar.pest"]
pub struct MetarParser;

#[derive(Debug)]
pub enum ParseError {
    MissingElement(String),
    MalformedInput(pest::error::Error<Rule>),
    Unknown(Box<dyn Error>),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Self::MalformedInput(value)
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(value: ParseFloatError) -> Self {
        Self::Unknown(Box::new(value))
    }
}

pub fn parse_metar(metar: &str) -> Result<Metar, ParseError> {
    let parsed = MetarParser::parse(Rule::METAR, metar)?.next().unwrap();

    let mut station = None;
    let mut observation_time = None;
    let mut automated_report = false;
    let mut wind = None;
    let mut visibility = None;
    let mut clouds = None;
    let mut temp = None;
    let mut dewpoint = None;

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

                wind = Some(metar::Wind {
                    direction: raw_direction.parse().unwrap(),
                    speed: raw_speed.parse().unwrap(),
                });
            }
            Rule::visibility => visibility = Some(pair.as_str().parse()?),
            Rule::clouds => clouds = Some(metar::Clouds::Clear),
            Rule::temp_dew => {
                let mut pairs = pair.into_inner();

                // Temperature and dewpoint always reported together.
                temp = parse_int_temp(pairs.next().unwrap());
                dewpoint = parse_int_temp(pairs.next().unwrap());
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
        visibility: visibility
            .ok_or_else(|| ParseError::MissingElement("Visibility".to_owned()))?,
        clouds: clouds.unwrap_or(metar::Clouds::Clear),
        temp: temp.ok_or_else(|| ParseError::MissingElement("Temperature".to_owned()))?,
        dewpoint: dewpoint.ok_or_else(|| ParseError::MissingElement("Dewpoint".to_owned()))?,
    })
}

fn parse_int_temp<'a>(pair: Pair<'a, Rule>) -> Option<i8> {
    match pair.as_rule() {
        Rule::temp_measurement => {
            let raw = pair.as_str();
            if raw.starts_with("M") {
                let parsed = raw[1..].parse::<i8>().ok();

                parsed.map(|value| -value)
            } else {
                raw.parse().ok()
            }
        }
        _ => None,
    }
}
