use std::ops::Deref;

/**
 * @brief small and fun exercise to create a regex builder
 * @author sorann753
 * @date 2023
 */

#[derive(Clone)]
enum RepeatType{
    AtLeast(usize),
    AtMost(usize),
    Exactly(usize),
    Between(usize, usize),
}

#[derive(Clone)]
enum RegexToken{
    None,
    Any,
    Number,
    Beginning,
    End,
    AnyLetter,
    UpperLetter,
    LowerLetter,
    Not(Box<RegexToken>),
    Character(char), //TODO : add escape characters to secure the regex
    AnyNumber(Box<RegexToken>),
    OneOrMore(Box<RegexToken>),
    Repeat(RepeatType, Box<RegexToken>),
    Optional(Box<RegexToken>),
    Group(Box<[RegexToken]>),
    Either(Box<RegexToken>, Box<RegexToken>),
}
impl RegexToken{
    fn to_string(&self) -> String {
        match self {
            RegexToken::None => String::from(""),
            RegexToken::Any => String::from("[^]"),
            RegexToken::Beginning => String::from("^"),
            RegexToken::End => String::from("$"),
            RegexToken::Number => String::from(r"\d"),
            RegexToken::UpperLetter => String::from(r"[A-Z]"),
            RegexToken::LowerLetter => String::from(r"[a-z]"),
            RegexToken::AnyLetter => String::from(r"[a-zA-Z]"),
            RegexToken::Character(c) => c.to_string(),

            RegexToken::AnyNumber(t) => format!("{}*", t.to_string()),
            RegexToken::OneOrMore(t) => format!("{}+", t.to_string()),
            RegexToken::Optional(t) => format!("({})?", t.to_string()),
            RegexToken::Repeat(repeat, token) => match repeat {
                RepeatType::AtLeast(n) => format!("{}{{{},}}", token.to_string(), n),
                RepeatType::AtMost(n) => format!("{}{{0,{}}}", token.to_string(), n),
                RepeatType::Exactly(n) => format!("{}{{{}}}", token.to_string(), n),
                RepeatType::Between(min, max) => format!("{}{{{},{}}}", token.to_string(), min, max),
            }

            RegexToken::Group(ts) => format!("({})", ts.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("")),
            RegexToken::Either(a, b) => format!("({}|{})", a.to_string(), b.to_string()),
            RegexToken::Not(_) => todo!("TODO : need to fix the logic")
        }
    }

    fn negate(&self) -> RegexToken {
        match self { //TODO verifier que tout marche bien
            RegexToken::None => RegexToken::Any,
            RegexToken::Any => RegexToken::None,
            RegexToken::Character(c) => RegexToken::Not(Box::new(RegexToken::Character(*c))),
            RegexToken::AnyNumber(t) => RegexToken::Not(Box::new(RegexToken::AnyNumber(t.clone()))),
            RegexToken::OneOrMore(t) => RegexToken::Not(Box::new(RegexToken::OneOrMore(t.clone()))),
            RegexToken::Optional(t) => RegexToken::Not(Box::new(RegexToken::Optional(t.clone()))),
            RegexToken::Group(ts) => RegexToken::Not(Box::new(RegexToken::Group(ts.clone()))),
            RegexToken::Either(a, b) => RegexToken::Not(Box::new(RegexToken::Either(a.clone(), b.clone()))),
            RegexToken::Number => RegexToken::Not(Box::new(RegexToken::Number)),
            RegexToken::Beginning => RegexToken::Not(Box::new(RegexToken::Beginning)),
            RegexToken::End => RegexToken::Not(Box::new(RegexToken::End)),
            RegexToken::AnyLetter => RegexToken::Not(Box::new(RegexToken::AnyLetter)),
            RegexToken::UpperLetter => RegexToken::Not(Box::new(RegexToken::UpperLetter)),
            RegexToken::LowerLetter => RegexToken::Not(Box::new(RegexToken::LowerLetter)),
            RegexToken::Repeat(repeat, token) => match repeat {
                RepeatType::AtLeast(n) => RegexToken::Not(Box::new(RegexToken::Repeat(RepeatType::AtLeast(*n), token.clone()))),
                RepeatType::AtMost(n) => RegexToken::Not(Box::new(RegexToken::Repeat(RepeatType::AtMost(*n), token.clone()))),
                RepeatType::Exactly(n) => RegexToken::Not(Box::new(RegexToken::Repeat(RepeatType::Exactly(*n), token.clone()))),
                RepeatType::Between(min, max) => RegexToken::Not(Box::new(RegexToken::Repeat(RepeatType::Between(*min, *max), token.clone()))),
            },
            RegexToken::Not(t) => t.deref().clone(),
        }
    }
}
impl Default for RegexToken{
    fn default() -> Self {
        RegexToken::None
    }
}

struct Regex {
    pattern: Vec<RegexToken>,
}
impl Regex{
    fn to_string(&self) -> String {
        self.pattern.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("")
    }
}

struct RegexBuilder;
impl RegexBuilder{
    fn new(args: Box<[RegexToken]>) -> Regex {
        if args.len() == 0 {
            Regex {
                pattern: Vec::new(),
            }
        }
        else{
            Regex {
                pattern: args.into_vec(),
            }
        }
    }

    fn start_of_string() -> RegexToken {
        return RegexToken::Beginning;
    }

    fn end_of_string() -> RegexToken {
        return RegexToken::End;
    }

    fn any() -> RegexToken {
        return RegexToken::Any;
    }

    fn number() -> RegexToken {
        return RegexToken::Number;
    }

    fn group(args: Box<[RegexToken]>) -> RegexToken {
        return RegexToken::Group(args);
    }

    fn any_number(token: RegexToken) -> RegexToken {
        return RegexToken::AnyNumber(Box::new(token));
    }

    fn at_least(n: usize, token: RegexToken) -> RegexToken {
        if n == 1 {
            return RegexToken::OneOrMore(Box::new(token));
        }
        else{
            return RegexToken::Repeat(RepeatType::AtLeast(n), Box::new(token));
        }
    }

    fn at_most(n: usize, token: RegexToken) -> RegexToken {
        return RegexToken::Repeat(RepeatType::AtMost(n), Box::new(token));
    }

    fn repeat(n: usize, token: RegexToken) -> RegexToken {
        return RegexToken::Repeat(RepeatType::Exactly(n), Box::new(token));
    }

    fn repeat_range(min: usize, max: usize, token: RegexToken) -> RegexToken {
        return RegexToken::Repeat(RepeatType::Between(min, max), Box::new(token));
    }

    fn optional(token : RegexToken) -> RegexToken {
        if let RegexToken::OneOrMore(t) = token {
            return RegexToken::AnyNumber(t);
        }

        return RegexToken::Optional(Box::new(token));
    }

    fn either(a: RegexToken, b: RegexToken) -> RegexToken {
        return RegexToken::Either(Box::new(a), Box::new(b));
    }

    fn character(c: char) -> RegexToken {
        return RegexToken::Character(c);
    }

    fn letter() -> RegexToken {
        return RegexToken::AnyLetter;
    }

    fn upper_letter() -> RegexToken {
        return RegexToken::UpperLetter;
    }

    fn lower_letter() -> RegexToken {
        return RegexToken::LowerLetter;
    }

    fn anything_except(a: RegexToken) -> RegexToken {
        return a.negate();
    }
}

macro_rules! RegexBuilder {
    ($($x:expr),*) => {
        RegexBuilder::new(Box::new([$($x),*]))
    };
}
macro_rules! RegexGroup {
    ($x: expr) => { // automatically remove useless groups
        $x
    };

    ($($x:expr),*) => {
        RegexBuilder::group(Box::new([$($x),*]))
    };
}

fn main() {
    let is_valid_order : Regex = RegexBuilder!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(5, RegexBuilder::number()),
        RegexGroup!(
            RegexBuilder::either(
                RegexGroup!(
                    RegexBuilder::at_least(1,RegexToken::Character('-')),
                    RegexBuilder::any_number(RegexBuilder::letter()),
                    RegexBuilder::at_least(1, RegexBuilder::number()),
                    RegexBuilder::end_of_string()
                ),
                RegexBuilder::end_of_string()
            )
        )
    );

    let is_valid_location = RegexBuilder!(
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

    let is_valid_order2 : Regex = RegexBuilder!(
        RegexBuilder::start_of_string(),
        RegexBuilder::at_least(5, RegexBuilder::number()),
        RegexBuilder::either(
            RegexGroup!(
                RegexBuilder::at_least(1, RegexToken::Character('-')),
                RegexBuilder::optional(
                    RegexBuilder::at_least(1, RegexBuilder::letter())
                ),
                RegexBuilder::at_least(1, RegexBuilder::number()),
                RegexBuilder::end_of_string()
            ),
            RegexBuilder::end_of_string()
        )
    );
    
    let is_valid_location2 = RegexBuilder!(
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
