use crate::prelude::*;

use crate::parser::*;

#[derive(PartialEq, Debug, Clone)]
enum ItemContent<'a> {
    DueDate(Period),
    Tag(Tag<'a>),
    Other(&'a str),
}

struct Item<'a> {
    checkbox: Checkbox,
    due_date: Period,
    priority: Option<Priority>,
    content: Vec<ItemContent<'a>>,
}

fn item_itemtag<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, ItemContent<'a>> {
    nom::combinator::map(item_tag(), |t| ItemContent::Tag(t))
}

fn item_duedate<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, ItemContent<'a>> {
    nom::combinator::map(due_date(), |d| ItemContent::DueDate(d))
}

fn item_other<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, ItemContent<'a>> {
    nom::combinator::map(
        take_till(|c| is_space(c as u8) || is_newline(c as u8)),
        |r| ItemContent::Other(r),
    )
}

fn item_description<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<ItemContent<'a>>> {
    separated_list1(
        alt((tag(" "), preceded(line_ending, tag("    ")))),
        alt((item_duedate(), item_itemtag(), item_other())),
    )
}

type Descs<'a> = Vec<ItemContent<'a>>;

type XItem<'a> = (Checkbox, Priority, Descs<'a>);

fn item_entry<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, XItem> {
    tuple((
        context("checkbox", checkbox()),
        preceded(context("Missing priority", tag(" ")), Priority::parse)
            .or(success(Priority::default())),
        preceded(
            tag(" "),
            context(
                "Too many due dates",
                verify(item_description(), |descs: &Descs| {
                    descs
                        .iter()
                        .filter(|content| matches!(content, ItemContent::DueDate(_)))
                        .count()
                        <= 1
                }),
            ),
        ),
    ))
}

fn item_list<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<XItem>> {
    terminated(
        separated_list1(many1(line_ending), item_entry()),
        line_ending,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check<'a, T>(res: IResult<&'a str, T>) -> bool {
        res.is_ok() && res.unwrap().0.len() == 0
    }

    #[test]
    fn test_basic() {
        assert!(check(item_entry()("[x] !!.. -> 2018-99-99 one two #tag")));
        assert!(check(item_entry()("[~] -> 2018-99-99 one #tag")));
        assert!((item_entry()("[ ] -> 2018-99-99 -> 2018-99-99")).is_err());
        assert!(check(dbg!(item_entry()("[x] one\n    two three"))));
    }

    #[test]
    fn test_list() {
        let input = r#"[ ] Open
[x] Checked
[@] Ongoing
[~] Obsolete
[ ] This is a longer ...
    description text
[ ] And this one ...
    is even ...
    longer
"#;
        assert!(check(dbg!(item_list()(input))));
    }
}
