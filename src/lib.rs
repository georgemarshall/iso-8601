mod test_readme {
    #[doc = include_str!("../README.md")]
    #[cfg(doctest)]
    pub struct ReadmeDoctests;
}

macro_rules! impl_fromstr_parse {
    ($ty:ty, $func:ident) => {
        impl ::std::str::FromStr for $ty {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                crate::parse::$func(s.as_bytes()).map(|x| x.1).or(Err(()))
            }
        }
    };
}

#[cfg(feature = "chrono")]
pub mod chrono;
mod date;
mod datetime;
mod parse;
mod time;

pub use crate::{date::*, datetime::*, time::*};

pub trait Valid {
    fn is_valid(&self) -> bool;
}
