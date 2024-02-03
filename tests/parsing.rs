use metars::{metar, Metar};

/// Test utility to parse a METAR that should be parseable, panicking if it
/// cannot be parsed.
fn parse_metar(metar: &str) -> Metar {
    metar.parse().expect("should be parseable")
}

#[test]
fn basic_metar() {
    let raw = "KTTA 031530Z AUTO 04008KT 10SM CLR 07/M02";
    let expected = Metar {
        station: "KTTA".to_owned(),
        observation_time: "031530Z".to_owned(),
        automated_report: true,
        wind: metar::Wind {
            direction: 40,
            speed: 8,
        },
        visibility: metar::Visibility::SM(10.0),
        clouds: metar::Clouds::Clear,
        temp: 7,
        dewpoint: -2,
    };

    let received: Metar = raw.parse().expect("should be parseable");
    println!("{:?}", received);

    assert_eq!(expected, received);
}

#[test]
fn fractional_visibility() {
    let metar = parse_metar("KTTA 031530Z AUTO 04008KT 1/2SM CLR 07/M02");

    assert_eq!(metar::Visibility::SM(0.5), metar.visibility);
}
