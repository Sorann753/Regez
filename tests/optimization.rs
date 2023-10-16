use regez::{MakeRegex, RegexGroup};

#[test]
fn either_common_token(){
    let rgx = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(5, RegexBuilder::number()),
        RegexGroup!(
            RegexBuilder::either(
                RegexGroup!(
                    RegexBuilder::at_least(1, RegexBuilder::character('-')),
                    RegexBuilder::any_number(RegexBuilder::letter()),
                    RegexBuilder::at_least(1, RegexBuilder::number()),
                    RegexBuilder::end_of_string()
                ),
                RegexBuilder::end_of_string()
            )
        )
    );

    let rgx_optimal = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(5, RegexBuilder::number()),
        RegexBuilder::optional(
            RegexGroup!(
                RegexBuilder::at_least(1, RegexBuilder::character('-')),
                RegexBuilder::any_number(RegexBuilder::letter()),
                RegexBuilder::at_least(1, RegexBuilder::number())
            ),
        ),
        RegexBuilder::end_of_string()
    );

    assert_eq!(rgx.to_string(), rgx_optimal.to_string());
    assert_eq!(rgx.to_string(), r"^\d{5,}(-+[a-zA-Z]*\d+)?$");
}
