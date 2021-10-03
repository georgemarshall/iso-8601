mod date;
mod datetime;
mod time;

pub use self::{date::*, datetime::*, time::*};

use nom::combinator::peek;
use nom::{
    branch::alt,
    character::streaming::{char, one_of},
    combinator::{map_opt, value},
    number::complete::recognize_float,
    IResult, ParseTo,
};
use std::ops::{AddAssign, MulAssign};

fn buf_to_int<T>(buf: &[u8]) -> T
where
    T: AddAssign + MulAssign + From<u8>,
{
    let mut sum = T::from(0);
    for digit in buf {
        sum *= T::from(10);
        sum += T::from(*digit - b'0');
    }
    sum
}

pub fn sign(i: &[u8]) -> IResult<&[u8], i8> {
    alt((value(-1, one_of("-\u{2212}\u{2010}")), value(1, char('+'))))(i)
}

fn frac32(i: &[u8]) -> IResult<&[u8], f32> {
    let (i, _) = peek(char('.'))(i)?;
    let (i, fraction) = map_opt(recognize_float, |s: &[u8]| s.parse_to())(i)?;
    Ok((i, fraction))
}

#[cfg(test)]
mod tests {
    use {
        nom::{
            error::{Error, ErrorKind::Char},
            Err,
            Needed::Size,
        },
        std::num::NonZeroUsize,
    };

    #[test]
    fn sign() {
        assert_eq!(super::sign(b"-"), Ok((&[][..], -1)));
        assert_eq!(super::sign(b"+"), Ok((&[][..], 1)));
        assert_eq!(
            super::sign(b""),
            Err(Err::Incomplete(Size(NonZeroUsize::new(1).unwrap())))
        );
        assert_eq!(
            super::sign(b" "),
            Err(Err::Error(Error {
                input: &b" "[..],
                code: Char
            }))
        );
    }
}
