
macro_rules! RegexBuilder {
    ($($x:expr),*) => {{
        type RegexBuilder = $crate::easy_regex::RegexBuilder::RegexBuilder;
        RegexBuilder::new(Box::new([$($x),*]))
    }};
}

macro_rules! RegexGroup {
    ($x: expr) => { // automatically remove useless groups
        $x
    };

    ($($x:expr),*) => {{
        type RegexBuilder = $crate::easy_regex::RegexBuilder::RegexBuilder;
        RegexBuilder::group(Box::new([$($x),*]))
    }};
}
