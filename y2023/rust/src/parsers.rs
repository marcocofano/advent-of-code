use std::iter::Filter;
use std::str::Split;

pub fn lines<'a>(input: &'a str) -> Filter<Split<'a, char>, fn(&&'a str) -> bool> {
    input.split('\n').filter(|l| !l.is_empty())
}
