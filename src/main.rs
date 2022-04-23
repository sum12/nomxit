mod parser;
mod prelude {
    pub use nom::branch::alt;
    pub use nom::bytes::complete::{tag, take, take_till};
    pub use nom::character::complete::{alpha1, char, one_of};
    pub use nom::character::is_digit;
    pub use nom::combinator::map_parser;
    pub use nom::combinator::not;
    pub use nom::combinator::peek;
    pub use nom::multi::{many0, many1};
    pub use nom::sequence::delimited;
    pub use nom::sequence::pair;
    pub use nom::sequence::preceded;
    pub use nom::sequence::separated_pair;
    pub use nom::sequence::terminated;
    pub use nom::sequence::tuple;
    pub use nom::IResult;
    pub use nom::Parser;
}

use parser::*;

fn main() {}
