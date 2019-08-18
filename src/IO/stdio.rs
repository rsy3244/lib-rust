use std::io::{stdin, Read, StdinLock};
use std::fmt;
use std::str::FromStr;

struct Scanner<'a> {
    cin : StdinLock<'a>,
}

impl<'a> Scanner<'a> {
    fn new(cin : StdinLock<'a>) -> Scanner<'a> {
        Scanner { cin: cin }
    }

    fn read1<T: FromStr>(&mut self) -> Option<T> {
        let token = self.cin.by_ref().bytes().map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        token.parse::<T>().ok()
    }

    fn read<T: FromStr>(&mut self) -> T {
        self.read1().unwrap()
    }
}

//aojのときに使う
/*
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseCharError {
    kind: CharErrorKind,
}

impl ParseCharError {
    pub fn __description(&self) -> &str {
        match self.kind {
            CharErrorKind::EmptyString => {
                "cannot parse char from empty string"
            },
            CharErrorKind::TooManyChars => "too many characters in string"
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CharErrorKind {
    EmptyString,
    TooManyChars,
}

impl fmt::Display for ParseCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl FromStr for char {
    type Err = ParseCharError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match (chars.next(), chars.next()) {
            (None, _) => {
                Err(ParseCharError { kind: CharErrorKind::EmptyString })
            },
            (Some(c), None) => Ok(c),
            _ => {
                Err(ParseCharError { kind: CharErrorKind::TooManyChars })
            }
        }
    }
}
*/
