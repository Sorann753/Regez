use crate::regex_type;
use crate::regex_type::Negate;

pub struct Regez {
    pub(crate) pattern: Box<[regex_type::RegexToken]>,
}
impl Regez{
    pub fn to_string(&self) -> String {
        self.pattern
            .iter()
            .map(|t| t.to_string())
            .collect::<Box<[String]>>()
            .join("")
    }

    pub(crate) fn optimize(self) -> Self{
        match self{ //TODO : place special case optimizations here
            _ => (),
        }
        return self;
    }

    // TODO
    pub fn negate(&self) -> Regez {
        let mut new_pattern = Vec::new();
        for token in self.pattern.iter() {
            new_pattern.push(token.negate());
        }
        Regez {
            pattern: new_pattern.into_boxed_slice(),
        }
    }
}
