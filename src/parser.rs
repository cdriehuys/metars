use pest::Parser;
use pest_derive::Parser;

use crate::Metar;

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

    for record in parsed.into_inner() {
        match record.as_rule() {
            Rule::station => {
                station = Some(record.as_str().to_owned());
            }
            Rule::observation_time => {
                observation_time = Some(record.as_str().to_owned());
            }
            Rule::auto_kw => {
                automated_report = true;
            }
            Rule::wind => {
                wind = Some(record.as_str().to_owned());
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
