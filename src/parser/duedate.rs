use crate::prelude::*;

/*
    yyyy-mm-dd or yyyy/mm/dd to reference a calendar day
    yyyy-mm or yyyy/mm to reference a month period
    yyyy to reference a year period
    yyyy-Www or yyyy/Www to reference a week period
    yyyy-Qq or yyyy/Qq to reference a quarter period
*/
#[derive(Debug, PartialEq, Clone)]
pub enum Period {
    Day((u16, u8, u8)),
    Month((u16, u8)),
    Year(u16),
    Week((u16, u8)),
    Quarter((u16, u8)),
}

fn year<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u16> {
    map_parser(take(4u8), nom::character::complete::u16)
}

fn month<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8> {
    preceded(
        tag("-"),
        map_parser(take(2u8), nom::character::complete::u8),
    )
}

fn day<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8> {
    preceded(
        tag("-"),
        map_parser(take(2u8), nom::character::complete::u8),
    )
}

fn week<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8> {
    preceded(
        tag("-W"),
        map_parser(take(2u8), nom::character::complete::u8),
    )
}

fn quarter<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8> {
    preceded(
        tag("-Q"),
        map_parser(take(1u8), nom::character::complete::u8),
    )
}

fn due_day<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period> {
    nom::combinator::map(tuple((year(), month(), day())), |(year, month, day)| {
        Period::Day((year, month, day))
    })
}

fn due_month<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period> {
    nom::combinator::map(tuple((year(), month())), |(year, month)| {
        Period::Month((year, month))
    })
}

fn due_year<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period> {
    nom::combinator::map(year(), |year| Period::Year(year))
}

fn due_week<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period> {
    nom::combinator::map(tuple((year(), week())), |(year, week)| {
        Period::Week((year, week))
    })
}

fn due_quarter<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period> {
    nom::combinator::map(tuple((year(), quarter())), |(year, quarter)| {
        Period::Quarter((year, quarter))
    })
}

pub fn due_date<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period> {
    preceded(
        tag("-> "),
        alt((
            due_day(),
            due_week(),
            due_quarter(),
            due_month(),
            due_year(),
        )),
    )
}

fn parse(input: &str) -> IResult<&str, Period> {
    //     let (input, year) = take(4u8)
    //         .and_then(nom::character::complete::u16)
    //         .parse(input)?;
    nom::combinator::map(tuple((year(), month(), day())), |(year, month, day)| {
        Period::Day((year, month, day))
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_basic() {
        assert_eq!(
            Ok(("", Period::Day((2018, 12, 12)))),
            due_day()("2018-12-12")
        );
        assert_eq!(Ok(("", Period::Month((2018, 12)))), due_month()("2018-12"));
        assert_eq!(Ok(("", Period::Year(2018))), due_year()("2018"));
        assert_eq!(
            Ok(("", Period::Quarter((2018, 1)))),
            due_quarter()("2018-Q1")
        );
        assert_eq!(Ok(("", Period::Week((2018, 52)))), due_week()("2018-W52"));
    }
}
