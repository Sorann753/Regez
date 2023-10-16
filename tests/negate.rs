use regez::{MakeRegex, RegexGroup};
use regex::Regex;

#[test]
fn basic_negate(){
    let rgx = MakeRegex!(
        RegexBuilder::not(RegexGroup!(
            RegexBuilder::start_of_string(),
            RegexBuilder::at_least(1, RegexBuilder::number()),
            RegexBuilder::at_least(2, RegexBuilder::letter()),
            RegexBuilder::end_of_string()
        ))
    );

    let rgx2 = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::not(RegexGroup!(
            RegexBuilder::at_least(1, RegexBuilder::number()),
            RegexBuilder::at_least(2, RegexBuilder::letter())
        )),
        RegexBuilder::end_of_string()
    );

    let positive_rgx = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(1, RegexBuilder::number()),
        RegexBuilder::at_least(2, RegexBuilder::letter()),
        RegexBuilder::end_of_string()
    );

    assert_eq!(rgx.to_string(), rgx2.to_string());
    assert_eq!(rgx.to_string(), r"^\D[a-zA-Z]?$");
    
    let negative = Regex::new(&rgx.to_string()).unwrap();
    let positive = Regex::new(&positive_rgx.to_string()).unwrap();
    let text = "6aB";
    assert_ne!(positive.is_match(text), negative.is_match(text));
}
