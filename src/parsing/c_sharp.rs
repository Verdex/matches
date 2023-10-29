
use std::str::Chars;

use renounce::*;
use structuralize::data::*;

// TODO : use structualize to process result?

pub fn parse(input : &str) -> Result<Data, ()> { // TODO error type?
    let mut input = input.chars();
    let result = parse_c_sharp(&mut input);
    match result {
        Ok(data) => Ok(Data::List(data)),
        Err(_) => Err(()), 
    }
}

macro_rules! opt {
    ($parser : ident => $optional : ident) => {
        fn $optional(input : &mut Chars) -> Result<Option<Data>, ParseError> {
            Ok(Some($parser(input)?))
        }
    };
}

fn parse_c_sharp(input : &mut Chars) -> Result<Vec<Data>, ParseError> {
    opt!(parse_keyword => o_parse_keyword);
    opt!(parse_id => o_parse_id);
    opt!(parse_block => o_parse_block);
    opt!(parse_paren => o_parse_paren);
    pat!(o_parse_dot: char => Option<Data> = '.' => { Some(Data::SymStr(SymStr::Symbol("dot".into())))});
    pat!(o_parse_colon: char => Option<Data> = ':' => { Some(Data::SymStr(SymStr::Symbol("colon".into())))});
    fn o_parse_arrow(input : &mut Chars) -> Result<Option<Data>, ParseError> {
        pat!(parse_eq: char => () = '=' => { () });
        pat!(parse_gt: char => () = '>' => { () });
        parser!(input => {
            _eq <= parse_eq;
            _gt <= parse_gt;
            select Some(Data::SymStr(SymStr::Symbol("arrow".into())))
        })
    }

    fn ignore(input : &mut Chars) -> Result<Option<Data>, ParseError> {
        parser!(input => {
            any <= parse_any;
            // TODO if square brackets are added then ']' will need to be added
            where any != ')' && any != '}';
            select None
        })
    }
    fn parse_item(input : &mut Chars) -> Result<Option<Data>, ParseError> {
        alt!(input => o_parse_keyword
                    ; o_parse_id 
                    ; o_parse_block
                    ; o_parse_paren
                    ; o_parse_dot
                    ; o_parse_colon
                    ; o_parse_arrow
                    ; ignore
                    )
    }

    parser!(input => {
        items <= * parse_item;
        select items.into_iter().filter_map(|x| x).collect()
    })
}

pat!(parse_any<'a>: char => char = x => x);

fn parse_word(input : &mut Chars) -> Result<Box<str>, ParseError> {
    fn parse_init_id_char(input : &mut Chars) -> Result<char, ParseError> {
        parser!(input => {
            any <= parse_any;
            where any.is_alphabetic() || any == '_';
            select any
        })
    }

    fn parse_rest_id_char(input : &mut Chars) -> Result<char, ParseError> {
        parser!(input => {
            any <= parse_any;
            where any.is_alphanumeric() || any == '_';
            select any
        })
    }

    parser!(input => {
        init <= parse_init_id_char;
        rest <= * parse_rest_id_char;
        select {
            let mut rest = rest;
            rest.insert(0, init);
            rest.into_iter().collect::<String>().into()
        }
    })
}

fn parse_keyword(input : &mut Chars) -> Result<Data, ParseError> {
    parser!(input => {
        word <= parse_word;
        where KEYWORDS.iter().find(|x| ***x == *word).is_some();
        select Data::Cons { name: "keyword".into(), params: vec![Data::SymStr(SymStr::String(word))] }
    })
}

fn parse_block(input : &mut Chars) -> Result<Data, ParseError> {
    pat!(parse_l_curl: char => () = '{' => { () });
    pat!(parse_r_curl: char => () = '}' => { () });

    parser!(input => {
        _l_curl <= parse_l_curl;
        items <= parse_c_sharp;
        _r_curl <= parse_r_curl;
        select Data::Cons { name: "block".into(), params: vec![Data::List(items)] }
    })
}

fn parse_generic(input : &mut Chars) -> Result<Data, ParseError> {

    fn parse_inside_generic(input : &mut Chars) -> Result<Data, ParseError> {
        opt!(parse_keyword => o_parse_keyword);
        opt!(parse_id => o_parse_id);
        pat!(o_parse_dot: char => Option<Data> = '.' => { Some(Data::SymStr(SymStr::Symbol("dot".into())))});
        pat!(o_parse_colon: char => Option<Data> = ':' => { Some(Data::SymStr(SymStr::Symbol("colon".into())))});

        fn ignore(input : &mut Chars) -> Result<Option<Data>, ParseError> {
            parser!(input => {
                any <= parse_any;
                where any != '>';
                select None
            })
        }
        fn parse_item(input : &mut Chars) -> Result<Option<Data>, ParseError> {
            alt!(input => o_parse_keyword
                        ; o_parse_id 
                        ; o_parse_dot
                        ; o_parse_colon
                        ; ignore
                        )
        }

        parser!(input => {
            items <= * parse_item;
            select Data::List(items.into_iter().filter_map(|x| x).collect())
        })
    }

    pat!(parse_l_angle: char => () = '<' => { () });
    pat!(parse_r_angle: char => () = '>' => { () });

    parser!(input => {
        _l_angle <= parse_l_angle;
        items <= parse_inside_generic;
        _r_angle <= parse_r_angle;
        select items
    })
}

fn parse_id(input : &mut Chars) -> Result<Data, ParseError> {
    // TODO unicode escape 
    pat!(parse_at: char => () = '@' => { () });

    parser!(input => {
        at <= ? parse_at;
        let _at : Option<()> = at;
        word <= parse_word;
        generic <= ? parse_generic;
        select {
            let mut params = vec![];
            params.push(Data::Cons { name: "name".into(), params: vec![Data::SymStr(SymStr::String(word))] });
            if let Some(generic) = generic {
                params.push(Data::Cons { name: "generic".into(), params: vec![generic] });
            }
            Data::Cons { name: "id".into(), params: vec![Data::List(params)] }
        }
    })
}

fn parse_paren(input : &mut Chars) -> Result<Data, ParseError> {
    pat!(parse_l_paren: char => () = '(' => { () });
    pat!(parse_r_paren: char => () = ')' => { () });

    parser!(input => {
        _l_paren <= parse_l_paren;
        items <= parse_c_sharp;
        _r_paren <= parse_r_paren;
        select Data::Cons { name: "paren".into(), params: vec![Data::List(items)] }
    })
}

static KEYWORDS : [&'static str; 120] =
    [   
        "as",
        "by",
        "do",
        "if",
        "in",
        "is",
        "on",
        "or",
        "add",
        "and",
        "for",
        "get",
        "int",
        "let",
        "new",
        "not",
        "out",
        "ref",
        "set",
        "try",
        "var",
        "args",
        "base",
        "bool",
        "byte",
        "case",
        "char",
        "else",
        "enum",
        "file",
        "from",
        "goto",
        "init",
        "into",
        "join",
        "lock",
        "long",
        "nint",
        "null",
        "this",
        "true",
        "uint",
        "void",
        "when",
        "with",
        "alias",
        "async",
        "await",
        "break",
        "catch",
        "class",
        "const",
        "event",
        "false",
        "fixed",
        "float",
        "group",
        "nuint",
        "sbyte",
        "short",
        "throw",
        "ulong",
        "using",
        "value",
        "where",
        "while",
        "yield",
        "double",
        "equals",
        "extern",
        "global",
        "nameof",
        "object",
        "params",
        "public",
        "record",
        "remove",
        "return",
        "scoped",
        "sealed",
        "select",
        "sizeof",
        "static",
        "string",
        "struct",
        "switch",
        "typeof",
        "unsafe",
        "ushort",
        "checked",
        "decimal",
        "default",
        "dynamic",
        "finally",
        "foreach",
        "managed",
        "notnull",
        "orderby",
        "partial",
        "private",
        "virtual",
        "abstract",
        "continue",
        "delegate",
        "explicit",
        "implicit",
        "internal",
        "operator",
        "override",
        "readonly",
        "required",
        "volatile",
        "ascending",
        "interface",
        "namespace",
        "protected",
        "unchecked",
        "unmanaged",
        "descending",
        "stackalloc",
    ];

#[cfg(test)]
mod test {
    use super::*;

    use structuralize::pattern::*;
    use structuralize::pattern::check::*;

    #[test]
    fn parse_id_should_parse() {
        let input = "@_SomeInput786";
        let mut input = input.chars();
        let output = parse_id(&mut input).unwrap();
        println!("{}", output);

        let p : Pattern = "id([name(\"_SomeInput786\")])".parse().unwrap();
        let p = check_pattern(p).unwrap();
        let results = pattern_match(&p, &output);

        assert_eq!( results.len(), 1 );
    }
}