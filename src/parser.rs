use std::{error::Error, num::ParseFloatError};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    metar::{CloudLayer, Clouds, Remarks, Visibility, Wind},
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

/// Builder for constructing a METAR over time.
///
/// Because a METAR has a set of required elements, we need some way of
/// collecting all the parsed values until we have all the information needed to
/// construct the final object.
#[derive(Default)]
struct MetarBuilder {
    station: Option<String>,
    observation_time: Option<String>,
    automated_report: bool,
    wind: Option<Wind>,
    visibility: Option<Visibility>,
    clouds: Option<Clouds>,
    temp: Option<i8>,
    dewpoint: Option<i8>,
    altimeter: Option<u16>,
    remarks: Option<Remarks>,
}

impl MetarBuilder {
    fn set_station(&mut self, station: String) {
        self.station = Some(station);
    }

    fn set_observation_time(&mut self, observation_time: String) {
        self.observation_time = Some(observation_time);
    }

    fn set_automated_report(&mut self, automated_report: bool) {
        self.automated_report = automated_report;
    }

    fn set_wind(&mut self, wind: Wind) {
        self.wind = Some(wind);
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = Some(visibility);
    }

    fn set_clouds(&mut self, clouds: Clouds) {
        self.clouds = Some(clouds);
    }

    fn set_temp(&mut self, temp: i8) {
        self.temp = Some(temp);
    }

    fn set_dewpoint(&mut self, dewpoint: i8) {
        self.dewpoint = Some(dewpoint);
    }

    fn set_altimeter(&mut self, altimeter: u16) {
        self.altimeter = Some(altimeter);
    }

    fn set_remarks(&mut self, remarks: Remarks) {
        self.remarks = Some(remarks);
    }

    fn build(self) -> Result<Metar, ParseError> {
        Ok(Metar {
            station: Self::required(self.station, "Station Name")?,
            observation_time: Self::required(self.observation_time, "Observation Time")?,
            automated_report: self.automated_report,
            wind: Self::required(self.wind, "Wind")?,
            visibility: Self::required(self.visibility, "Visibility")?,
            clouds: self.clouds.unwrap_or(Clouds::Clear),
            temp: Self::required(self.temp, "Temperature")?,
            dewpoint: Self::required(self.dewpoint, "Dewpoint")?,
            altimeter: Self::required(self.altimeter, "Altimeter")?,
            remarks: self.remarks,
        })
    }

    /// Require an optional attribute of the builder to be set, and return a
    /// [`ParseError`] if it isn't.
    fn required<T>(param: Option<T>, name: &str) -> Result<T, ParseError> {
        param.ok_or_else(|| ParseError::MissingElement(name.to_owned()))
    }
}

pub fn parse_metar(metar: &str) -> Result<Metar, ParseError> {
    let parsed = MetarParser::parse(Rule::METAR, metar)?.next().unwrap();

    let mut builder = MetarBuilder::default();

    for pair in parsed.into_inner() {
        match pair.as_rule() {
            Rule::station => {
                builder.set_station(pair.as_str().to_owned());
            }
            Rule::observation_time => {
                builder.set_observation_time(pair.as_str().to_owned());
            }
            Rule::auto_kw => {
                builder.set_automated_report(true);
            }
            Rule::wind => {
                // Wind is defined as a direction followed by a speed.
                let mut pairs = pair.into_inner();

                let raw_direction = pairs.next().unwrap().as_str();
                let raw_speed = pairs.next().unwrap().as_str();

                // If there is another element, it must be the gust speed.
                let gust_speed = pairs.next().map(|gusting| {
                    gusting
                        .as_str()
                        .strip_prefix('G')
                        .unwrap()
                        .parse::<u8>()
                        .unwrap()
                });

                builder.set_wind(Wind {
                    direction: raw_direction.parse().unwrap(),
                    speed: raw_speed.parse().unwrap(),
                    gust_speed,
                });
            }
            Rule::visibility => builder.set_visibility(pair.as_str().parse()?),
            Rule::clouds => {
                // Clouds are either a keyword indicating the sky is clear or
                // a list of cloud layers, but it's always a single element.
                let cloud_pair = pair.into_inner().next().unwrap();

                match cloud_pair.as_rule() {
                    Rule::clouds_clr => builder.set_clouds(Clouds::Clear),
                    Rule::cloud_layers => {
                        let mut layers = Vec::new();

                        for layer in cloud_pair.into_inner() {
                            let mut layer_pairs = layer.into_inner();

                            let layer_name = layer_pairs.next().unwrap().as_str();
                            let layer_height = layer_pairs.next().unwrap().as_str();

                            // The grammar constrains the names and heights to
                            // parseable values, so we can `unwrap` here.
                            layers.push(CloudLayer {
                                kind: layer_name.parse().unwrap(),
                                // Cloud heights specified in hundreds.
                                agl: layer_height.parse::<u16>().unwrap() * 100,
                            });
                        }

                        builder.set_clouds(Clouds::Layers(layers));
                    }
                    _ => unreachable!(),
                }
            }
            Rule::temp_dew => {
                let mut pairs = pair.into_inner();

                // Temperature and dewpoint always reported together.
                builder.set_temp(parse_int_temp(pairs.next().unwrap()).unwrap());
                builder.set_dewpoint(parse_int_temp(pairs.next().unwrap()).unwrap());
            }
            Rule::altimeter => {
                // Since the grammar constrains us to a 4 digit number with an
                // 'A' prefix, we can unwrap the results here.
                let numeric = pair.as_str().strip_prefix('A').unwrap();
                builder.set_altimeter(numeric.parse().unwrap());
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

                builder.set_remarks(Remarks { station_type })
            }
            _ => unreachable!(),
        }
    }

    builder.build()
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
