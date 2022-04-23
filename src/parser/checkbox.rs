use crate::prelude::*;
#[derive(Debug, PartialEq)]
pub enum CheckType {
    Space,
    Checked,
    At,
    Tilde,
}

impl CheckType {
    pub fn parse(input: &str) -> IResult<&str, Self> {
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
pub struct Checkbox(pub CheckType);

impl Checkbox {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("[")(input)?;
        let (input, checktype) = CheckType::parse(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, Self(checktype)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basi() {
        assert_eq!(
            Checkbox::parse("[x]"),
            Ok(("", Checkbox(CheckType::Checked)))
        );
    }
}
