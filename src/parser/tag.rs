use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Tag<'a> {
    Name(&'a str),
    KeyVal((&'a str, &'a str)),
}

fn name<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Tag, VerboseError<&str>> {
    map(
        terminated(preceded(tag("#"), alpha1), nom::combinator::not(tag("="))),
        |name| Tag::Name(name),
    )
}

fn middle<'a>(delim: char) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, VerboseError<&str>> {
    delimited(
        nom::character::complete::char(delim),
        take_till(move |c| c == delim),
        nom::character::complete::char(delim),
    )
}

fn keyval<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Tag, VerboseError<&str>> {
    map(
        context(
            "failed to parse tag",
            preceded(
                tag("#"),
                separated_pair(
                    context("unable to parse tag key", alpha1),
                    tag("="),
                    context(
                        "Unable to parse tag value",
                        alt((middle('"'), middle('\''), alpha1)),
                    ),
                ),
            ),
        ),
        |(k, v)| Tag::KeyVal((k, v)),
    )
}

pub fn item_tag<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Tag, VerboseError<&str>> {
    alt((name(), keyval()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Ok(("", Tag::Name("tagname"))), name()("#tagname"));
        assert_eq!(Ok(("", Tag::Name("tagname"))), item_tag()("#tagname"));
        assert_eq!(Ok(("", Tag::KeyVal(("key", "val")))), keyval()("#key=val"));
        assert_eq!(
            Ok(("", Tag::KeyVal(("key", "val")))),
            item_tag()("#key=val")
        );
        assert_eq!(
            Ok(("", Tag::KeyVal(("key", "v a l")))),
            item_tag()("#key='v a l'")
        );
        assert_eq!(
            Ok(("", Tag::KeyVal(("key", "v a l")))),
            item_tag()("#key=\"v a l\"")
        );
    }
}
