use nom::{bytes::complete::tag, character::complete::one_of, IResult};

#[derive(Debug, PartialEq)]
enum CheckType {
    Space,
    Checked,
    At,
    Tilde,
}

impl CheckType {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, chr) = one_of(" @x~")(input)?;
        Ok((
            input,
            match chr {
                ' ' => CheckType::Space,
                'x' => CheckType::Checked,
                '@' => CheckType::At,
                '~' => CheckType::Tilde,
                _ => {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::OneOf,
                    )))
                }
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
struct Checkbox(CheckType);

impl Checkbox {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("[")(input)?;
        let (input, checktype) = CheckType::parse(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, Self(checktype)))
    }
}

fn main() {
    assert_eq!(
        Checkbox::parse("[x]"),
        Ok(("", Checkbox(CheckType::Checked)))
    );
}
