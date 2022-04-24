mod parser;
mod prelude {
    pub use nom::branch::alt;
    pub use nom::bytes::complete::{tag, take, take_till};
    pub use nom::character::complete::{alpha1, char, line_ending, one_of, space1};
    pub use nom::character::{is_digit, is_newline, is_space};
    pub use nom::combinator::all_consuming;
    pub use nom::combinator::cond;
    pub use nom::combinator::fail;
    pub use nom::combinator::map_parser;
    pub use nom::combinator::not;
    pub use nom::combinator::peek;
    pub use nom::combinator::success;
    pub use nom::combinator::verify;
    pub use nom::error::context;
    pub use nom::error::VerboseError;
    pub use nom::multi::fold_many1;
    pub use nom::multi::{count, many0, many1, many_m_n, many_till, separated_list1};
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
