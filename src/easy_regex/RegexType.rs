/**
 * @brief File defining the enum types and their implementations
 * @author Sorann753
 * @date 2023
 */

use std::ops::Deref;

pub trait negate{
    fn negate(&self) -> Self;
}

#[derive(Clone)]
pub enum RepeatType{
    AtLeast(usize),
    AtMost(usize),
    Between(usize, usize),
    Exactly(usize),
    AnyNumberExcept(usize),
    Either(Box<RepeatType>, Box<RepeatType>), //TODO : find another way to negate the Between without having to add this
}
impl negate for RepeatType{
    fn negate(&self) -> Self {
        match self {
            RepeatType::AtLeast(n) => RepeatType::AtMost(*n - 1),
            RepeatType::AtMost(n) => RepeatType::AtLeast(*n + 1),
            RepeatType::Exactly(n) => RepeatType::AnyNumberExcept(*n),
            RepeatType::AnyNumberExcept(n) => RepeatType::Exactly(*n),
            RepeatType::Between(min, max) if min == &(0 as usize) => {
                RepeatType::AtMost(*max).negate()
            },
            RepeatType::Between(min, max) => {
                RepeatType::Either(
                    Box::new(RepeatType::AtMost(*min - 1)),
                    Box::new(RepeatType::AtLeast(*max + 1))
                )
            },
            RepeatType::Either(a, b) => match (a, b){ //TODO : if we keep this, use this match to make special case optimizations
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
                RepeatType::AnyNumberExcept(n) => format!("{}{{{}}}", token.to_string(), n),
                RepeatType::Between(min, max) => format!("{}{{{},{}}}", token.to_string(), min, max),
            }

            RegexToken::Group(ts) => format!("({})", ts.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("")),
            RegexToken::Either(a, b) => format!("({}|{})", a.to_string(), b.to_string()),
            RegexToken::Not(_) => todo!("TODO : need to fix the logic")
        }
    }
}
impl Default for RegexToken{
    fn default() -> Self {
        RegexToken::None
    }
}
impl negate for RegexToken{
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
            RegexToken::Repeat(repeat, token) => RegexToken::Repeat(repeat.negate(), token.clone()),
            RegexToken::Not(t) => *t.clone(),
        }
    }
}

pub struct Regex {
    pub pattern: Box<[RegexToken]>,
}
impl Regex{
    pub fn to_string(&self) -> String {
        self.pattern
            .iter()
            .map(|t| t.to_string())
            .collect::<Box<[String]>>()
            .join("")
    }
}
