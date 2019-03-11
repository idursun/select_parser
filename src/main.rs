#[macro_use]
extern crate nom;
use nom::types::{CompleteByteSlice};
use nom::{space1};

#[derive(Debug)]
struct Statement<'a> {
    columns: Vec<&'a str>,
    table: &'a str,
}

fn complete_byte_slice_to_str(s: CompleteByteSlice) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(s.0)
}

fn is_ident(c: u8) -> bool {
    nom::is_alphanumeric(c) || nom::is_alphabetic(c)
}

named!(ident<CompleteByteSlice, CompleteByteSlice>, take_while!(is_ident) );

named!(ident_list<CompleteByteSlice, Vec<&str>>,
    separated_list!(
        char!(','),
        map_res!(
            ws!(ident),
            complete_byte_slice_to_str
        )
    )
);

named!(table_name<CompleteByteSlice, &str>,
    map_res!(
        delimited!(
            char!('['),
            take_until!("]"),
            char!(']')
        ),
    complete_byte_slice_to_str
));

named!(p_select<CompleteByteSlice, Statement>,
    do_parse!(
        tag!("select") >>
        space1 >>
        columns: ident_list >>
        tag!("from") >>
        space1 >>
        table: table_name >>
        (Statement { columns, table })
    )
);

fn main() {
    match p_select(CompleteByteSlice(b"select a ,  b ,   c from [table1]")) {
        Ok(result) => println!("{:?}", result),
        Err(e) => eprintln!("{:?}", e)
    };
}
