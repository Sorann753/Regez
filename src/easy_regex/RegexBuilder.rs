use crate::easy_regex::RegexType::*;

pub struct RegexBuilder;
impl RegexBuilder{
    pub fn new(args: Box<[RegexToken]>) -> Regex {
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

    pub fn start_of_string() -> RegexToken {
        return RegexToken::Beginning;
    }

    pub fn end_of_string() -> RegexToken {
        return RegexToken::End;
    }

    pub fn any() -> RegexToken {
        return RegexToken::Any;
    }

    pub fn number() -> RegexToken {
        return RegexToken::Number;
    }

    pub fn group(args: Box<[RegexToken]>) -> RegexToken {
        return RegexToken::Group(args);
    }

    pub fn any_number(token: RegexToken) -> RegexToken {
        return RegexToken::AnyNumber(Box::new(token));
    }

    pub fn at_least(n: usize, token: RegexToken) -> RegexToken {
        if n == 1 {
            return RegexToken::OneOrMore(Box::new(token));
        }
        else{
            return RegexToken::Repeat(RepeatType::AtLeast(n), Box::new(token));
        }
    }

    pub fn at_most(n: usize, token: RegexToken) -> RegexToken {
        return RegexToken::Repeat(RepeatType::AtMost(n), Box::new(token));
    }

    pub fn repeat(n: usize, token: RegexToken) -> RegexToken {
        return RegexToken::Repeat(RepeatType::Exactly(n), Box::new(token));
    }

    pub fn repeat_range(min: usize, max: usize, token: RegexToken) -> RegexToken {
        return RegexToken::Repeat(RepeatType::Between(min, max), Box::new(token));
    }

    pub fn optional(token : RegexToken) -> RegexToken {
        if let RegexToken::OneOrMore(t) = token {
            return RegexToken::AnyNumber(t);
        }

        return RegexToken::Optional(Box::new(token));
    }

    pub fn either(a: RegexToken, b: RegexToken) -> RegexToken {
        return RegexToken::Either(Box::new(a), Box::new(b));
    }

    pub fn character(c: char) -> RegexToken {
        return RegexToken::Character(c);
    }

    pub fn letter() -> RegexToken {
        return RegexToken::AnyLetter;
    }

    pub fn upper_letter() -> RegexToken {
        return RegexToken::UpperLetter;
    }

    pub fn lower_letter() -> RegexToken {
        return RegexToken::LowerLetter;
    }

    pub fn anything_except(a: RegexToken) -> RegexToken {
        return a.negate();
    }
}
