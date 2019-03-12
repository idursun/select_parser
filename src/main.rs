#[macro_use]
extern crate nom;
use nom::{space1, space0, multispace};
use nom::types::CompleteByteSlice;

#[derive(Debug)]
struct Statement<'a> {
    columns: Vec<Object<'a>>,
    table: Object<'a>,
}

#[derive(Debug)]
struct Object<'a> {
    schema: Option<&'a str>,
    name: &'a str,
}

fn complete_byte_slice_to_str(s: CompleteByteSlice) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(s.0)
}

fn is_ident(c: u8) -> bool {
    nom::is_alphanumeric(c) || nom::is_alphabetic(c)
}

named!(ident<CompleteByteSlice, CompleteByteSlice>, take_while!(is_ident) );

named!(delimited_object<CompleteByteSlice, CompleteByteSlice>, 
        delimited!(
            char!('['), 
            take_until!("]"), 
            char!(']')
        ) 
);

named!(p_object<CompleteByteSlice, Object>, 
    do_parse!(  
        schema: opt!(
                    map_res!( 
                        do_parse!(
                            s: alt!(delimited_object | ident) >> 
                            char!('.') >> 
                            (s)) 
                        , complete_byte_slice_to_str)) >> 
        name: map_res!(alt!(delimited_object | ident), complete_byte_slice_to_str) >> 
        (Object {schema, name})
    )
);

named!(ident_list<CompleteByteSlice, Vec<Object>>, separated_list!( char!(','), ws!(p_object)));

named!(p_select<CompleteByteSlice, Statement>,
    do_parse!(
        tag!("select") >>
        multispace >>
        columns: ident_list >>
        tag!("from") >>
        multispace >>
        table: p_object >>
        (Statement { columns, table })
    )
);

fn main() {
    match p_select(CompleteByteSlice(b"select a ,  b ,   c from [dbo].[table1]")) {
        Ok(result) => println!("{:?}", result),
        Err(e) => eprintln!("{:?}", e),
    };
}
