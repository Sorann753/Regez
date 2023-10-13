use regez::MakeRegex;
use regex::Regex;

#[test]
fn simple_test() {
    let rgx = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(1, RegexBuilder::number()),
        RegexBuilder::end_of_string()
    );

    assert_eq!(rgx.to_string(), r"^\d+$");

    let re = Regex::new(&rgx.to_string()).unwrap();
    let text = "123";

    assert!(re.is_match(text));
}