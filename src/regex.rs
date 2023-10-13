use crate::regex_type;

pub struct Regex {
    pub pattern: Box<[regex_type::RegexToken]>,
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
