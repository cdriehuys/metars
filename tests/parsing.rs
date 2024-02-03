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
    let parsed = parse_metar("KTTA 031530Z AUTO 04008KT 1/2SM CLR 07/M02");

    assert_eq!(metar::Visibility::SM(0.5), parsed.visibility);
}

#[test]
fn clouds_few() {
    let parsed = parse_metar("KRDU 031751Z 03006KT 10SM FEW015 11/M03 A3005");
    let want_clouds = metar::Clouds::Layers(vec![metar::CloudLayer {
        kind: metar::CloudKind::Few,
        agl: 1500,
    }]);

    assert_eq!(want_clouds, parsed.clouds);
}

#[test]
fn clouds_scattered() {
    let parsed = parse_metar("KRDU 031751Z 03006KT 10SM SCT050 11/M03 A3005");
    let want_clouds = metar::Clouds::Layers(vec![metar::CloudLayer {
        kind: metar::CloudKind::Scattered,
        agl: 5000,
    }]);

    assert_eq!(want_clouds, parsed.clouds);
}

#[test]
fn clouds_all() {
    let parsed = parse_metar("KBOS 031954Z 03006KT 10SM BKN010 FEW020 SCT025 OVC055 11/M03 A3005");
    let want_clouds = metar::Clouds::Layers(vec![
        metar::CloudLayer {
            kind: metar::CloudKind::Broken,
            agl: 1000,
        },
        metar::CloudLayer {
            kind: metar::CloudKind::Few,
            agl: 2000,
        },
        metar::CloudLayer {
            kind: metar::CloudKind::Scattered,
            agl: 2500,
        },
        metar::CloudLayer {
            kind: metar::CloudKind::Overcast,
            agl: 5500,
        },
    ]);

    assert_eq!(want_clouds, parsed.clouds);
}
