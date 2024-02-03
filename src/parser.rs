use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "metar.pest"]
pub struct MetarParser;
