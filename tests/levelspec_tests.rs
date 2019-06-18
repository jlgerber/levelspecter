use levelspecter::prelude::*;

#[test]
fn can_parse_shot() {
    let ls = LevelSpec::from_str("DEV01.RD.9999");
    assert_eq!(ls, Ok(LevelSpec::from_shot("DEV01", "RD", "9999")));
}


#[test]
fn can_parse_shot_with_wildcard() {
    let ls = LevelSpec::from_str("DEV01.%.9999");
    assert_eq!(ls, Ok(LevelSpec::from_shot("DEV01", "%", "9999")));
}

#[test]
fn can_parse_seq() {
    let ls = LevelSpec::from_str("DEV01.RD");
    assert_eq!(ls, Ok(LevelSpec::from_sequence("DEV01", "RD")));
}

#[test]
fn can_parse_show() {
    let ls = LevelSpec::from_str("DEV01");
    assert_eq!(ls, Ok(LevelSpec::from_show("DEV01")));
}