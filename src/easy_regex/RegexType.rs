use std::ops::Deref;

#[derive(Clone)]
pub enum RepeatType{
    AtLeast(usize),
    AtMost(usize),
    Exactly(usize),
    Between(usize, usize),
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
                RepeatType::Between(min, max) => format!("{}{{{},{}}}", token.to_string(), min, max),
            }

            RegexToken::Group(ts) => format!("({})", ts.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("")),
            RegexToken::Either(a, b) => format!("({}|{})", a.to_string(), b.to_string()),
            RegexToken::Not(_) => todo!("TODO : need to fix the logic")
        }
    }

    pub fn negate(&self) -> RegexToken {
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

pub struct Regex {
    pub pattern: Vec<RegexToken>,
}
impl Regex{
    pub fn to_string(&self) -> String {
        self.pattern.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("")
    }
}
