use regez::{MakeRegex, RegexGroup};
use regex::Regex;
mod common;

#[test]
fn negate_basic_token(){

    // check the negation of basic tokens
    assert_eq!( MakeRegex!(RegexBuilder::start_of_string()).negate().to_string(),"^" );
    assert_eq!( MakeRegex!(RegexBuilder::end_of_string()).negate().to_string(),"$" );
    assert_eq!( MakeRegex!(RegexBuilder::any()).negate().to_string(),r"\n" );
    assert_eq!( MakeRegex!(RegexBuilder::number()).negate().to_string(),r"\D" );
}

#[test]
fn negate_repeat_token(){

    // check the negation of the quantifier and alternation kind of tokens
    assert_eq!( MakeRegex!(RegexBuilder::any_number(RegexBuilder::number())).negate().to_string(),r"\D*" );
}

#[test]
fn negate_complex_token(){

    // check the negation of more complex tokens
    
}



#[test]
fn basic_negate_case(){
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
        &rgx.to_string(),
        &rgx2.to_string(),
        &rgx3.to_string(),
        &r"^\D[a-zA-Z]?$".to_string()
    ]);
    
    let negative = Regex::new(&rgx.to_string()).unwrap();
    let positive = Regex::new(&positive_rgx.to_string()).unwrap();
    let text = "6aB";
    let text2 = "3X";
    let text3 = "lorem";

    common::assert_ne_on_texts(negative, positive, &[text, text2, text3]);
}
