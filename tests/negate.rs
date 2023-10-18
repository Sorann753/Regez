use regez::{MakeRegex, RegexGroup};
use regex::Regex;
mod common;

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

    let rgx3 = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::anything_except(RegexGroup!(
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

    common::assert_all_eq(&[
        rgx.to_string(),
        rgx2.to_string(),
        rgx3.to_string(),
        r"^\D[a-zA-Z]?$".to_string()
    ]);
    
    let negative = Regex::new(&rgx.to_string()).unwrap();
    let positive = Regex::new(&positive_rgx.to_string()).unwrap();
    let text = "6aB";
    let text2 = "3X";
    let text3 = "lorem";

    common::assert_ne_on_texts(negative, positive, Box::new([text, text2, text3]));
}
