use regez::{MakeRegex, RegexGroup};
use regex::Regex;
mod common;

#[test]
fn at_least_1_number() {
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

#[test]
fn either_test(){
    let rgx = MakeRegex!(
        RegexBuilder::either(
            RegexGroup!(
                RegexBuilder::at_least(1, RegexBuilder::number()),
                RegexBuilder::at_least(2, RegexBuilder::letter()),
                RegexBuilder::character('X')
            ),
            RegexGroup!(
                RegexBuilder::repeat_exactly(3, RegexBuilder::character('W')),
                RegexBuilder::any_number(RegexBuilder::any())
            )
        )
    );

    assert_eq!(rgx.to_string(), r"(\d+[a-zA-Z]{2}X)|(WWW.*)");

    let from_regez = Regex::new(&rgx.to_string()).unwrap();
    let from_string = Regex::new(&rgx.to_string()).unwrap();
    let text1 = "547WWWlmaoX";
    let text2 = "547abX";
    let text3 = "WWWlmao";

    common::assert_eq_on_texts(from_regez, from_string, Box::new([text1, text2, text3]));
}
