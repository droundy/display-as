use combine::{Parser, Stream, ParseError, between, many, many1, any, look_ahead,
              skip_count, not_followed_by};
use combine::parser::char::{string};
use combine::parser::repeat::{take_until};
use combine::parser::item::{eof};
use combine::stream::state::State;

#[derive(Eq, Debug, PartialEq)]
enum Template {
    RawString(String),
    Expression(String),
}


fn template_parser<I>() -> impl Parser<Input = I, Output = Vec<Template>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    template_parser_()
}

// We need to use `parser!` to break the recursive use of `value` to
// prevent the returned parser from containing itself
parser!{
    #[inline(always)]
    fn template_parser_[I]()(I) -> Vec<Template>
        where [ I: Stream<Item = char> ]
    {
        let raw_ends = string("{{").map(|_| ())
            .or(string("{%").map(|_| ()))
            .or(eof());
        let raw = look_ahead(any())
            .with(take_until(raw_ends).map(|x| Template::RawString(x)));
        let expression = between(string("{{"), string("}}"), take_until(string("}}")))
            .map(|s| Template::Expression(s));
        // let mut expression = between(string("{{"), string("}}"), template_parser::<I>())
        //     .map(|s| Template::Expression(s.to_string()));
        many(expression.or(raw))
    }
}

fn parse_template(s: &str) -> Vec<Template> {
    match template_parser().easy_parse(State::new(s)) {
        Err(e) => {
            panic!("ran into an error {:?}", e);
        }
        Ok(v) => {
            v.0
        }
    }
}

#[test]
fn parse_simple_string() {
    assert_eq!(parse_template("hello world"),
               vec![Template::RawString("hello world".to_string())]);
    assert_eq!(parse_template("{{hello world}}"),
               vec![Template::Expression("hello world".to_string())]);
    assert_eq!(parse_template("hello {{world}}"),
               vec![Template::RawString("hello ".to_string()),
                    Template::Expression("world".to_string())]);
    assert_eq!(parse_template("hello {{world}}!"),
               vec![Template::RawString("hello ".to_string()),
                    Template::Expression("world".to_string()),
                    Template::RawString("!".to_string()),
               ]);
}
