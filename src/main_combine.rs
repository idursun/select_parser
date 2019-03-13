use combine::char::space;
use combine::parser::range::{range, take_while1};
use combine::stream::state::State;
use combine::{
    attempt, between, choice, optional, sep_by1, skip_many, token, ParseError, Parser, RangeStream,
};

#[derive(Debug, PartialEq)]
struct Statement<'a> {
    columns: ColumnList<'a>,
    table: Object<'a>,
}

#[derive(Debug, PartialEq)]
struct Object<'a> {
    schema: Option<&'a str>,
    name: &'a str,
}

#[derive(Debug, PartialEq)]
enum ColumnList<'a> {
    All,
    List(Vec<Object<'a>>),
}

fn ident<'a, I>() -> impl Parser<Input = I, Output = &'a str>
where
    I: RangeStream<Item = char, Range = &'a str>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    take_while1(|c: char| c.is_alphabetic())
}

fn delimited_ident<'a, I>() -> impl Parser<Input = I, Output = &'a str>
where
    I: RangeStream<Item = char, Range = &'a str>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(token('['), token(']'), ident())
}

fn p_object_name<'a, I>() -> impl Parser<Input = I, Output = Object<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        skip_many(space()),
        optional(delimited_ident().skip(token('.'))),
        choice((attempt(delimited_ident()), ident())),
        skip_many(space()),
    )
        .map(|(_, schema, name, _)| Object { schema, name })
}

fn column_list<'a, I>() -> impl Parser<Input = I, Output = ColumnList<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    attempt(token('*').map(|_| ColumnList::All))
    .or(sep_by1::<Vec<_>, _, _>( p_object_name(), token(','), ) .map(|list| ColumnList::List(list)))
}

fn p_select<'a, I>() -> impl Parser<Input = I, Output = Statement<'a>>
where
    I: RangeStream<Item = char, Range = &'a str>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        range("select"),
        skip_many(space()),
        column_list(),
        skip_many(space()),
        range("from"),
        p_object_name(),
    )
        .map(|(_, _, columns, _, _, table)| Statement { columns, table })
}

fn test(input: &str) {
    match p_select().easy_parse(State::new(input)) {
        Ok((output, _remaining)) => println!("{:?}", output),
        Err(err) => eprintln!("{}", err),
    };
}

fn main() {
    test("select * from table1");
    test("select a,  b,c,d from table1");
}
