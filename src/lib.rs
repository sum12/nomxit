pub mod parser;
pub mod prelude {
    pub use nom::branch::alt;
    pub use nom::bytes::complete::{tag, take, take_till};
    pub use nom::character::complete::{alpha1, char, line_ending, one_of, space0, space1};
    pub use nom::character::{is_digit, is_newline, is_space};
    pub use nom::combinator::{
        all_consuming, cond, fail, map, map_parser, not, opt, peek, success, verify,
    };
    pub use nom::error::{context, VerboseError};
    pub use nom::multi::{count, fold_many1, many0, many1, many_m_n, many_till, separated_list1};
    pub use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
    pub use nom::{IResult, Parser};
}

use parser::*;
