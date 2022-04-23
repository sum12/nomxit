use nom::character::complete::char;
use nom::multi::{many0, many1};
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum PriorityComponent {
    Padding(usize),
    Exclaimation(usize),
}
#[derive(Debug, PartialEq)]
pub struct Priority((PriorityComponent, PriorityComponent));

impl Priority {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let de = pair(
            many1(char('.')).map(|v| PriorityComponent::Padding(v.len())),
            many0(char('!')).map(|v| PriorityComponent::Exclaimation(v.len())),
        );
        let ed = pair(
            many1(char('!')).map(|v| PriorityComponent::Exclaimation(v.len())),
            many0(char('.')).map(|v| PriorityComponent::Padding(v.len())),
        );
        de.or(ed)
            .parse(input)
            .and_then(|(input, (com1, com2))| Ok((input, Priority((com1, com2)))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Priority::parse("!..."),
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
