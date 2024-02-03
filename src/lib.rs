use std::str::FromStr;

use parser::Rule;
use pest::Parser;

mod parser;

#[derive(Debug, PartialEq)]
pub struct Metar {
    pub station: String,
}

impl FromStr for Metar {
    type Err = pest::error::Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = parser::MetarParser::parse(Rule::METAR, s)?.next().unwrap();

        let mut station = "".to_owned();
        for record in parsed.into_inner() {
            match record.as_rule() {
                Rule::station => {
                    station = record.as_str().to_owned();
                },
                _ => unreachable!()
            }
        }

        Ok(Metar{station})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_metar() {
        let raw = "KTTA 031530Z AUTO 04008KT 10SM CLR 07/M02";
        let expected = Metar{station: "KTTA".to_owned()};

        let received: Metar = raw.parse().expect("should be parseable");

        assert_eq!(expected, received);
    }
}
