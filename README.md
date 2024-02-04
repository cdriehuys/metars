# metars

[![Build status](https://github.com/cdriehuys/metars/actions/workflows/rust.yml/badge.svg)](https://github.com/cdriehuys/metars/actions/workflows/rust.yml)

Rust library for parsing METAR strings.

For now, this is mostly a toy project for learning about the plethora of
elements that can be in a METAR, as well as how to parse structured text in
Rust. It is also very focused on reports from the US which have some different
elements and structure than their ICAO counterparts.

If you really want to use this library, it can be installed directly from
GitHub:

```toml
metars = { git = "https://github.com/cdriehuys/metars.git" }
```

## METAR Reference

*__Note:__ This is by no means a comprehensive reference. These are just some of
the components of a METAR relevant to parsing them.*

A METAR has a base set of required elements. For example:
```
KEWR 031951Z 01012KT 10SM CLR 08/M06 A3002
```

Broken down, this reads as:
* **Station Name:** The reporting station is "KEWR" (Newark Airport)
* **Report Time:** Reported on the 3rd of the month, at 1951Z (UTC)
* **Winds:** From 10 degrees at 12 knots
* **Visibility:** 10 statute miles
* **Sky Conditions:** Skies are clear
* **Temperature/Dewpoint:** Temperature is 8 C, dewpoint is -6 C
* **Altimeter Setting:** 30.02 inHg

### Sky Conditions

In the example above, the skies are reported `CLR` (clear). This gets more
interesting when there are cloud layers. Cloud layers are reported as a type
and height above ground level. For example, this METAR has two cloud layers:
```
KRDU 041351Z 00000KT 10SM FEW150 SCT250 04/00 A3005
```

1. A layer of few clouds at 15,000 ft AGL, and
2. A layer of scattered clouds at 25,000 ft AGL

### Remarks

Remarks are truly where the wheels come off. Following all the standard
elements in the METAR, there can be a keyword `RMK` followed by any number of
remarks adding information to the report.

Here are a few elements that may be present.

#### Temperature Breakdown

A temperature breakdown remark provides the temperature and dewpoint to tenths
of a degree rather than the integer report that's always provided. For example:
```
T10281100
```

The temperature and dewpoint each use 4 digits. The first digit in each group
indicates the sign of the temperature: `0` for positive, `1` for negative. The
remaining digits indicate tenths of a degree. In this case, the temperature is
-2.8 C, and the doewpoint is -10.0 C.
