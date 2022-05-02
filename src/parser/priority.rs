// use nom::character::complete::char;
// use nom::multi::{many0, many1};
// use nom::sequence::pair;
// use nom::IResult;
// use nom::Parser;
use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PriorityComponent {
    Padding(usize),
    Exclaimation(usize),
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Priority((PriorityComponent, PriorityComponent));

impl std::default::Default for Priority {
    fn default() -> Self {
        Priority((
            PriorityComponent::Exclaimation(0usize),
            PriorityComponent::Padding(0usize),
        ))
    }
}

pub fn priority<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Priority, VerboseError<&str>> {
    let de = pair(
        map(many1(char('.')), |v| PriorityComponent::Padding(v.len())),
        map(many0(char('!')), |v| {
            PriorityComponent::Exclaimation(v.len())
        }),
    );
    let ed = pair(
        map(many1(char('!')), |v| {
            PriorityComponent::Exclaimation(v.len())
        }),
        map(many0(char('.')), |v| PriorityComponent::Padding(v.len())),
    );
    map(alt((de, ed)), |(com1, com2)| (Priority((com1, com2))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            priority()("!..."),
            Ok((
                "",
                Priority((
                    PriorityComponent::Exclaimation(1),
                    PriorityComponent::Padding(3)
                ))
            ))
        );
    }
}
