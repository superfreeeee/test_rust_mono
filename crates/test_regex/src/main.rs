use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_regex() {
    let email_reg = Regex::new(
        r"(?x)
        ^(?P<login>[^@\s]+)@
        ([[:word:]]+\.)*
        [[:word:]]+$
        ",
    )
    .unwrap();

    println!("{:?}", extract_login(&email_reg, r"I❤email@example.com"));
}

#[allow(dead_code)]
fn extract_login<'a>(re: &Regex, input: &'a str) -> Option<&'a str> {
    re.captures(input)
        .and_then(|cap| cap.name("login").map(|login| login.as_str()))
}

#[test]
fn test_hashtags() {
    let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
    let tags = extract_hashtags(tweet);
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
    assert_eq!(tags.len(), 3);
}

#[allow(dead_code)]
fn extract_hashtags(text: &str) -> HashSet<&str> {
    lazy_static! {
        static ref HASHTAG_REGEX: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
    }
    HASHTAG_REGEX
        .find_iter(text)
        .map(|mat| mat.as_str())
        .collect()
}
