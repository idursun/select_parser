#[macro_use]
extern crate nom;
use nom::multispace;
use nom::types::CompleteByteSlice;

#[derive(Debug, PartialEq)]
struct Statement<'a> {
    columns: Vec<Object<'a>>,
    table: Object<'a>,
}

#[derive(Debug, PartialEq)]
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
    match p_select(CompleteByteSlice(
        b"select a ,  b ,   c from [dbo].[table1]",
    )) {
        Ok(result) => println!("{:?}", result),
        Err(e) => eprintln!("{:?}", e),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ident() {
        let result = ident(CompleteByteSlice(b"ident"));
        assert_eq!(
            result,
            Ok((CompleteByteSlice(&b""[..]), CompleteByteSlice(b"ident")))
        );
    }

    #[test]
    fn test_object_name() {
        let result = p_object(CompleteByteSlice(b"[a].[b]"));
        assert_eq!(
            result,
            Ok((
                CompleteByteSlice(&b""[..]),
                Object {
                    schema: Some("a"),
                    name: "b"
                }
            ))
        );
    }

    #[test]
    fn test_statement() {
        let result = p_select(CompleteByteSlice(
            b"select a ,  b ,   c from [dbo].[table1]",
        ));
        let expected = Statement {
            table: Object {
                schema: Some("dbo"),
                name: "table1",
            },
            columns: vec![
                Object {
                    schema: None,
                    name: "a",
                },
                Object {
                    schema: None,
                    name: "b",
                },
                Object {
                    schema: None,
                    name: "c",
                },
            ],
        };
        assert_eq!(result, Ok((CompleteByteSlice(&b""[..]), expected)));
    }
}
