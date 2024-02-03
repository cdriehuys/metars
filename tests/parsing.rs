use metars::Metar;

#[test]
fn basic_metar() {
    let raw = "KTTA 031530Z AUTO 04008KT 10SM CLR 07/M02";
    let expected = Metar {
        station: "KTTA".to_owned(),
        observation_time: "031530Z".to_owned(),
        automated_report: true,
        wind: "04008KT".to_owned(),
    };

    let received: Metar = raw.parse().expect("should be parseable");

    assert_eq!(expected, received);
}
