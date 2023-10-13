
#[macro_export]
macro_rules! MakeRegex {
    ($($x:expr),*) => {{
        type RegexBuilder = $crate::regex_builder::RegexBuilder;
        RegexBuilder::new(Box::new([$($x),*]))
    }};
}

#[macro_export]
macro_rules! RegexGroup {
    ($x: expr) => { // automatically remove useless groups
        $x
    };

    ($($x:expr),*) => {{
        type RegexBuilder = $crate::regex_builder::RegexBuilder;
        RegexBuilder::group(Box::new([$($x),*]))
    }};
}
