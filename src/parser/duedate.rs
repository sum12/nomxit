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

fn year<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u16, VerboseError<&str>> {
    map_parser(take(4u8), nom::character::complete::u16)
}

fn month<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8, VerboseError<&str>> {
    preceded(
        tag("-"),
        map_parser(take(2u8), nom::character::complete::u8),
    )
}

fn day<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8, VerboseError<&str>> {
    preceded(
        tag("-"),
        map_parser(take(2u8), nom::character::complete::u8),
    )
}

fn week<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8, VerboseError<&str>> {
    preceded(
        tag("-W"),
        map_parser(take(2u8), nom::character::complete::u8),
    )
}

fn quarter<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u8, VerboseError<&str>> {
    preceded(
        tag("-Q"),
        map_parser(take(1u8), nom::character::complete::u8),
    )
}

fn due_day<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period, VerboseError<&str>> {
    map(tuple((year(), month(), day())), |(year, month, day)| {
        Period::Day((year, month, day))
    })
}

fn due_month<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period, VerboseError<&str>> {
    map(tuple((year(), month())), |(year, month)| {
        Period::Month((year, month))
    })
}

fn due_year<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period, VerboseError<&str>> {
    map(year(), |year| Period::Year(year))
}

fn due_week<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period, VerboseError<&str>> {
    map(tuple((year(), week())), |(year, week)| {
        Period::Week((year, week))
    })
}

fn due_quarter<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period, VerboseError<&str>> {
    map(
        context("unable to parse quarter", tuple((year(), quarter()))),
        |(year, quarter)| Period::Quarter((year, quarter)),
    )
}

pub fn due_date<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Period, VerboseError<&str>> {
    preceded(
        tag("-> "),
        context(
            "Cannot parse due date",
            alt((
                due_day(),
                due_week(),
                due_quarter(),
                due_month(),
                due_year(),
            )),
        ),
    )
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
