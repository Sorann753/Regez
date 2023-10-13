/**
 * @brief File defining the enum types and their implementations
 * @author Sorann753
 * @date 2023
 */

use std::ops::Deref;

pub trait Negate{
    fn negate(&self) -> Self;
}



#[derive(Clone)]
pub enum RepeatType{
    AtLeast(i32),
    AtMost(i32),
    Between(i32, i32),
    Exactly(i32),
    AnyNumberExcept(i32),
    Either(Box<RepeatType>, Box<RepeatType>), //TODO : find another way to negate the Between without having to add this
}
impl Negate for RepeatType{
    fn negate(&self) -> Self {
        match self {
            RepeatType::AtLeast(n) => RepeatType::AtMost(*n - 1),
            RepeatType::AtMost(n) => RepeatType::AtLeast(*n + 1),
            RepeatType::Exactly(n) => RepeatType::AnyNumberExcept(*n),
            RepeatType::AnyNumberExcept(n) => RepeatType::Exactly(*n),
            RepeatType::Between(min, max) => match (min, max) {
                (min, max) if min == max => RepeatType::AnyNumberExcept(*min),
                (min, max) if min == &0 => RepeatType::AtLeast(*max + 1),
                _ => RepeatType::Either(
                    Box::new(RepeatType::AtMost(*min - 1)),
                    Box::new(RepeatType::AtLeast(*max + 1))
                )
            },
            RepeatType::Either(a, b) => match (a.deref(), b.deref()){ //TODO : if we keep the Either, use this match to make special case optimizations
                (RepeatType::AtLeast(n), RepeatType::AtMost(m)) if n-1 > m+1 => RepeatType::Between(*m + 1, *n - 1),
                (RepeatType::AtLeast(n), RepeatType::AtMost(m)) if n-1 == m+1 => RepeatType::Exactly(n-1),
                (RepeatType::AtLeast(n), RepeatType::AtMost(m)) if n-1 < m+1 => RepeatType::Exactly(0),
                _ => RepeatType::Either(
                    Box::new(a.negate()),
                    Box::new(b.negate())
                ),
            }
        }
    }
}



#[derive(Clone)]
pub enum RegexToken{
    None,
    Any,
    Beginning,
    End,
    Number,
    NotANumber,
    AnyLetter,
    UpperLetter, //TODO : add UTF-8 option for the letter cases
    LowerLetter,
    NotALetter,
    Not(Box<RegexToken>),
    Character(char),
    AnyNumber(Box<RegexToken>),
    OneOrMore(Box<RegexToken>),
    Repeat(RepeatType, Box<RegexToken>),
    Optional(Box<RegexToken>),
    Group(Box<[RegexToken]>),
    Either(Box<RegexToken>, Box<RegexToken>),
    // Both(Box<RegexToken>, Box<RegexToken>),
}
impl RegexToken{
    pub fn to_string(&self) -> String {
        match self {
            RegexToken::None => String::from(""),
            RegexToken::Any => String::from("[^]"),
            RegexToken::Beginning => String::from("^"),
            RegexToken::End => String::from("$"),
            RegexToken::Number => String::from(r"\d"),
            RegexToken::NotANumber => String::from(r"\D"),
            RegexToken::UpperLetter => String::from(r"[A-Z]"),
            RegexToken::LowerLetter => String::from(r"[a-z]"),
            RegexToken::AnyLetter => String::from(r"[a-zA-Z]"),
            RegexToken::NotALetter => String::from(r"[^a-zA-Z]"),
            RegexToken::Character(c) => RegexToken::escape_char(*c),

            RegexToken::AnyNumber(t) => format!("{}*", t.to_string()),
            RegexToken::OneOrMore(t) => format!("{}+", t.to_string()),
            RegexToken::Optional(t) => format!("({})?", t.to_string()),
            RegexToken::Repeat(repeat, token) => match repeat {
                RepeatType::AtLeast(n) => format!("{}{{{},}}", token.to_string(), n),
                RepeatType::AtMost(n) => format!("{}{{0,{}}}", token.to_string(), n),
                RepeatType::Exactly(n) => format!("{}{{{}}}", token.to_string(), n),
                RepeatType::AnyNumberExcept(n) => format!("{}{{{}}}", token.to_string(), n),
                RepeatType::Between(min, max) => format!("{}{{{},{}}}", token.to_string(), min, max),
                RepeatType::Either(a, b) => format!("({}|{})", RegexToken::Repeat(a.deref().clone(), token.clone()).to_string(), RegexToken::Repeat(b.deref().clone(), token.clone()).to_string()),
            }

            RegexToken::Group(ts) => format!("({})", ts.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("")),
            RegexToken::Either(a, b) => format!("({}|{})", a.to_string(), b.to_string()),
            RegexToken::Not(t) => t.negate().to_string(),
        }
    }

    fn negate_group(ts : &Box<[RegexToken]>) -> RegexToken {
        match ts { // place special case optimizations here
            _ => RegexToken::Group(ts.iter().map(|t| t.negate()).collect::<Box<[RegexToken]>>()),
        }
    }

    fn escape_char(c : char) -> String {
        match c { //TODO : finish escaping special char and test function
            '.' | '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\' => format!("\\{}", c),
            _ => format!("{}", c),
        }
    }
}
impl Default for RegexToken{
    fn default() -> Self {
        RegexToken::None
    }
}
impl Negate for RegexToken{
    fn negate(&self) -> RegexToken {
        match self {
            RegexToken::None => RegexToken::Any,
            RegexToken::Any => RegexToken::None,
            RegexToken::Character(_) => todo!("all char but one"),
            RegexToken::AnyNumber(_) => RegexToken::None,
            RegexToken::OneOrMore(t) => RegexToken::Repeat(RepeatType::Exactly(0), t.clone()),
            RegexToken::Optional(t) => t.deref().clone(),
            RegexToken::Group(ts) => RegexToken::negate_group(ts),
            RegexToken::Either(_, _) => todo!("find a way to negate either"),
            RegexToken::Number => RegexToken::NotANumber,
            RegexToken::NotANumber => RegexToken::Number,
            RegexToken::Beginning => RegexToken::End,
            RegexToken::End => RegexToken::Beginning,
            RegexToken::AnyLetter => RegexToken::NotALetter,
            RegexToken::NotALetter => RegexToken::AnyLetter,
            RegexToken::UpperLetter => todo!("find a way to negate UpperLetter"),
            RegexToken::LowerLetter => todo!("find a way to negate LowerLetter"),
            RegexToken::Repeat(repeat, token) => RegexToken::Repeat(repeat.negate(), token.clone()),
            RegexToken::Not(t) => t.deref().clone(),
        }
    }
}
