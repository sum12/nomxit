use crate::prelude::*;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CheckType {
    Space,
    Checked,
    At,
    Tilde,
}

#[derive(Debug, PartialEq)]
pub struct Checkbox(pub CheckType);

type CheckboxResult<'a> = IResult<&'a str, Checkbox>;

fn boxd<'a>(c: char, ret: CheckType) -> impl FnMut(&'a str) -> CheckboxResult {
    nom::combinator::map(
        delimited(
            nom::character::complete::char('['),
            nom::character::complete::char(c),
            nom::character::complete::char(']'),
        ),
        move |_| Checkbox(ret),
    )
}

fn type_space<'a>() -> impl FnMut(&'a str) -> CheckboxResult {
    boxd(' ', CheckType::Space)
}
fn type_checked<'a>() -> impl FnMut(&'a str) -> CheckboxResult {
    boxd('x', CheckType::Checked)
}
fn type_at<'a>() -> impl FnMut(&'a str) -> CheckboxResult {
    boxd('@', CheckType::At)
}
fn type_tilde<'a>() -> impl FnMut(&'a str) -> CheckboxResult {
    boxd('~', CheckType::Tilde)
}

pub fn checkbox<'a>() -> impl FnMut(&'a str) -> CheckboxResult {
    alt((type_space(), type_checked(), type_at(), type_tilde()))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basi() {
        assert_eq!(checkbox()("[x]"), Ok(("", Checkbox(CheckType::Checked))));
        assert_eq!(checkbox()("[@]"), Ok(("", Checkbox(CheckType::At))));
        assert_eq!(checkbox()("[ ]"), Ok(("", Checkbox(CheckType::Space))));
        assert_eq!(checkbox()("[~]"), Ok(("", Checkbox(CheckType::Tilde))));
    }
}
