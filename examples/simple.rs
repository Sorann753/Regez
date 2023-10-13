/**
 * @brief small and fun exercise to create a regex builder
 * @author sorann753
 * @date 2023
 */

use regez::{MakeRegex, RegexGroup, regex::*};

fn main() {
    let is_valid_order : Regex = MakeRegex!(
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

    let is_valid_location = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(1, RegexBuilder::letter()),
        RegexBuilder::character('-'),
        RegexBuilder::number(),
        RegexBuilder::repeat(2, RegexGroup!(
            RegexBuilder::character('-'),
            RegexBuilder::repeat(2, RegexBuilder::number())
        )),
        RegexBuilder::end_of_string()
    );

    let is_valid_order2 : Regex = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(5, RegexBuilder::number()),
        RegexBuilder::either(
            RegexGroup!(
                RegexBuilder::at_least(1, RegexBuilder::character('-')),
                RegexBuilder::optional(
                    RegexBuilder::at_least(1, RegexBuilder::letter())
                ),
                RegexBuilder::at_least(1, RegexBuilder::number()),
                RegexBuilder::end_of_string()
            ),
            RegexBuilder::end_of_string()
        )
    );
    
    let is_valid_location2 = MakeRegex!(
        RegexBuilder::start_of_string(),
        RegexBuilder::upper_letter(),
        RegexBuilder::any_number(RegexBuilder::either(
            RegexBuilder::upper_letter(),
            RegexBuilder::number()
        )),
        RegexBuilder::at_least(1, RegexBuilder::letter()),
        RegexBuilder::character('-'),
        RegexBuilder::number(),
        RegexBuilder::anything_except(RegexBuilder::number()),
        RegexBuilder::repeat(2, RegexGroup!(
            RegexBuilder::character('-'),
            RegexBuilder::repeat(2, RegexBuilder::number())
        )),
        RegexBuilder::end_of_string()
    );

    println!("is_valid_order : {}", is_valid_order.to_string());
    println!("is_valid_location : {}", is_valid_location.to_string());
    println!("is_valid_order2 : {}", is_valid_order2.to_string());
    println!("is_valid_location2 : {}", is_valid_location2.to_string());
}
