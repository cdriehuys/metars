use std::{error::Error, num::ParseFloatError};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    metar::{self, Clouds, Remarks},
    Metar,
};

#[derive(Parser)]
#[grammar = "metar.pest"]
pub struct MetarParser;

#[derive(Debug)]
pub enum ParseError {
    MissingElement(String),
    MalformedInput(Box<pest::error::Error<Rule>>),
    Unknown(Box<dyn Error>),
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Self::MalformedInput(Box::new(value))
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
    let mut altimeter = None;
    let mut remarks = None;

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
            Rule::clouds => {
                // Clouds are either a keyword indicating the sky is clear or
                // a list of cloud layers, but it's always a single element.
                let cloud_pair = pair.into_inner().next().unwrap();

                match cloud_pair.as_rule() {
                    Rule::clouds_clr => clouds = Some(Clouds::Clear),
                    Rule::cloud_layers => {
                        let mut layers = Vec::new();

                        for layer in cloud_pair.into_inner() {
                            let mut layer_pairs = layer.into_inner();

                            let layer_name = layer_pairs.next().unwrap().as_str();
                            let layer_height = layer_pairs.next().unwrap().as_str();

                            // The grammar constrains the names and heights to
                            // parseable values, so we can `unwrap` here.
                            layers.push(metar::CloudLayer {
                                kind: layer_name.parse().unwrap(),
                                // Cloud heights specified in hundreds.
                                agl: layer_height.parse::<u16>().unwrap() * 100,
                            });
                        }

                        clouds = Some(Clouds::Layers(layers));
                    }
                    _ => unreachable!(),
                }
            }
            Rule::temp_dew => {
                let mut pairs = pair.into_inner();

                // Temperature and dewpoint always reported together.
                temp = parse_int_temp(pairs.next().unwrap());
                dewpoint = parse_int_temp(pairs.next().unwrap());
            }
            Rule::altimeter => {
                // Since the grammar constrains us to a 4 digit number with an
                // 'A' prefix, we can unwrap the results here.
                let numeric = pair.as_str().strip_prefix('A').unwrap();
                altimeter = Some(numeric.parse().unwrap());
            }
            Rule::remarks => {
                let mut station_type = None;

                for remark in pair.into_inner() {
                    match remark.as_rule() {
                        Rule::remark_station_type => {
                            station_type = Some(remark.as_str().to_owned())
                        }
                        _ => unreachable!("Unknown pair: {:?}", remark),
                    }
                }

                remarks = Some(Remarks { station_type })
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
        altimeter: altimeter.ok_or_else(|| ParseError::MissingElement("Altimeter".to_owned()))?,
        remarks,
    })
}

fn parse_int_temp(pair: Pair<Rule>) -> Option<i8> {
    match pair.as_rule() {
        Rule::temp_measurement => {
            let raw = pair.as_str();
            if let Some(negative_value) = raw.strip_prefix('M') {
                let parsed = negative_value.parse::<i8>().ok();

                parsed.map(|value| -value)
            } else {
                raw.parse().ok()
            }
        }
        _ => None,
    }
}
