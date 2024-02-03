use metars::{metar, Metar};

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
    };

    let received: Metar = raw.parse().expect("should be parseable");
    println!("{:?}", received);

    assert_eq!(expected, received);
}

#[test]
fn fractional_visibility() {
    let raw = "KTTA 031530Z AUTO 04008KT 1/2SM CLR 07/M02";
    let received: Metar = raw.parse().expect("should be parseable");

    assert_eq!(metar::Visibility::SM(0.5), received.visibility);
}
