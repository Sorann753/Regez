use regez::regex::Regez;
use regex::Regex;

pub fn assert_eq_on_texts(from_regez: Regex, rgx: Regex, texts: &[&str]){
    for text in texts.iter(){
        println!("testing on \"{}\"", text);
        assert_eq!(from_regez.is_match(text), rgx.is_match(text));
    }
}

pub fn assert_ne_on_texts(from_regez: Regex, rgx: Regex, texts: &[&str]){
    for text in texts.iter(){
        println!("testing on \"{}\"", text);
        assert_ne!(from_regez.is_match(text), rgx.is_match(text));
    }
}

pub fn assert_all_eq<T>(input : &[&T]) where T: std::fmt::Debug + PartialEq
{
    for i in 0..input.len(){ //if [0] == [i] and [0] == [i+1] then [i] == [i+1] for all i
        assert_eq!(input[0], input[i]);
    }
}

/**
 * Assert that in a and b all pair a[i] != b[i] are true
 */
pub fn assert_arr_ne<T>(a : &[&T], b : &[&T]) where T: std::fmt::Debug + PartialEq
{
    for i in 0..a.len(){
        println!("testing #{}", i);
        assert_ne!(a[i], b[i]);
    }
}
