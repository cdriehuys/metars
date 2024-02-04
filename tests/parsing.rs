use metars::{metar, Metar};

/// Test utility to parse a METAR that should be parseable, panicking if it
/// cannot be parsed.
fn parse_metar(metar: &str) -> Metar {
    metar.parse().expect("should be parseable")
}

#[test]
fn basic_metar() {
    let raw = "KTTA 031530Z AUTO 04008KT 10SM CLR 07/M02 A3001";
    let expected = Metar {
        station: "KTTA".to_owned(),
        observation_time: "031530Z".to_owned(),
        automated_report: true,
        wind: metar::Wind {
            direction: 40,
            speed: 8,
            gust_speed: None,
        },
        visibility: metar::Visibility::SM(10.0),
        clouds: metar::Clouds::Clear,
        temp: 7,
        dewpoint: -2,
        altimeter: 3001,
        remarks: None,
    };

    let received: Metar = raw.parse().expect("should be parseable");
    println!("{:?}", received);

    assert_eq!(expected, received);
}

#[test]
fn winds_gusting() {
    let parsed = parse_metar("KBOS 031954Z 03006G19KT 10SM CLR 11/M03 A3005");

    assert_eq!(19, parsed.wind.gust_speed.unwrap());
}

#[test]
fn fractional_visibility() {
    let parsed = parse_metar("KTTA 031530Z AUTO 04008KT 1/2SM CLR 07/M02 A3001");

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

#[test]
fn remark_station_type_ao1() {
    let parsed = parse_metar("KEWR 031951Z 01012KT 10SM CLR 08/M06 A3002 RMK AO1");

    assert_eq!(Some("AO1".to_owned()), parsed.remarks.unwrap().station_type);
}

#[test]
fn remark_station_type_ao2() {
    let parsed = parse_metar("KEWR 031951Z 01012KT 10SM CLR 08/M06 A3002 RMK AO2");

    assert_eq!(Some("AO2".to_owned()), parsed.remarks.unwrap().station_type);
}

#[test]
fn remark_temp_detail() {
    let parsed =
        parse_metar("KRDU 041351Z 00000KT 10SM FEW150 SCT250 04/00 A3005 RMK AO2 T00440000");

    let temp_breakdown = parsed
        .remarks
        .expect("should have remarks")
        .temp_breakdown
        .expect("should have temp breakdown");

    assert_eq!(4.4, temp_breakdown.temp);
    assert_eq!(0.0, temp_breakdown.dewpoint);
}

#[test]
fn remark_temp_detail_negative() {
    let parsed =
        parse_metar("KRDU 041351Z 00000KT 10SM FEW150 SCT250 04/00 A3005 RMK AO2 T10441056");

    let temp_breakdown = parsed
        .remarks
        .expect("should have remarks")
        .temp_breakdown
        .expect("should have temp breakdown");

    assert_eq!(-4.4, temp_breakdown.temp);
    assert_eq!(-5.6, temp_breakdown.dewpoint);
}
