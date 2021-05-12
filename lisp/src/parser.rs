extern crate alloc;

use alloc::collections::btree_map::BTreeMap;
use alloc::fmt;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Define the AST for lisp programs (and lisp values as lisp is homoiconic).

/// We use references in oder to have fixed sized objects.
enum LispVal<'a> {
    Atom(&'a str),
    List(Vec<LispVal<'a>>),
    Number(isize),
    String(&'a str),
    Fun(&'a IFunc),
    Lambda(&'a IFunc, EnvCtx<'a>),
    Nil,
    Bool(bool),
}

/// Environment for evaluating expressions.
type EnvCtx<'a> = BTreeMap<&'a str, LispVal<'a>>;

/// Represents functions.
/// The context is needed for scoping.
struct IFunc {
    fun: dyn Fn(Vec<LispVal>) -> Option<(LispVal, EnvCtx)>,
}

/// Define a printer for [LispVal].
impl<'a> fmt::Display for LispVal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            LispVal::Atom(s) => write!(f, "{}", s),
            LispVal::List(l) => {
                let mut s = String::new();

                l.into_iter().for_each(|e| {
                    s.push_str((format!(" {}", e)).as_str());
                });
                write!(f, "({} )", s)
            }
            LispVal::Number(n) => write!(f, "{}", n),
            LispVal::String(s) => write!(f, "{}", s),
            LispVal::Fun(_) => write!(f, "( internal function )"),
            LispVal::Lambda(_, _) => write!(f, "( lambda function )"),
            LispVal::Nil => write!(f, "Nil"),
            LispVal::Bool(true) => write!(f, "#t"),
            LispVal::Bool(false) => write!(f, "#f"),
        }
    }
}

// Define the parser.

/// Unwrapped parser generic return type.
type UnwrappedParserResult<'a, A> = (&'a str, A);

/// Parser generic return type.
type ParserResult<'a, A> = Option<UnwrappedParserResult<'a, A>>;

/// Parser generic type.
type Parser<'a, A> = &'a dyn Fn(&'a str) -> ParserResult<'a, A>;

// Parser combinators.

/// Alternative parser combiner. Tries the rightmost parser first.
macro_rules! alt {
    ($s: expr ; $p: ident) => {
        $p($s)
    };

    ($s: expr ; $p: ident | $( $tail: ident )|* ) => {
        (alt! { $s ; $( $tail )|* }).or($p($s))
    };
}

/// Repeats a parser as much as possible and folds the results with `combine`.
// TODO FAUX
fn repeat<'a, A>(p: Parser<'a, A>, s: &'a str, combine: &dyn Fn(A, A) -> A) -> ParserResult<'a, A> {
    p(s).and_then(|actual_res: UnwrappedParserResult<A>| {
        repeat(p, actual_res.0, combine).map(|tail_res: UnwrappedParserResult<A>| {
            (tail_res.0, combine(actual_res.1, tail_res.1))
        })
    })
}

/// Create string parsers and combine them with [alt!].
/// Tries the rightmost parser fisrt.
macro_rules! alt_string_p {
    ( $s: expr ; $( $name: ident $string: literal )|* ) => {
        $(
            // I miss curried functions.
            fn $name(s: &str) -> ParserResult<&str> { string_p($string, s) }
        )*
        alt! { $s ; $( $name )|* }
    };
}

// Parsers.

/// Parses a given string.
fn string_p<'a>(s_p: &'a str, s: &'a str) -> ParserResult<'a, &'a str> {
    match s.strip_prefix(s_p) {
        // TODO .map
        None => None,
        Some(tail) => Some((tail, s_p)),
    }
}

/// Parses [LispVal::Nil].
fn lisp_nil(s: &str) -> ParserResult<LispVal> {
    string_p("Nil", s).map(|(tail, _)| (tail, LispVal::Nil))
}

// Booleans.

/// Parses `#t`.
fn true_p(s: &str) -> ParserResult<LispVal> {
    string_p("#t", s).map(|(tail, _)| (tail, LispVal::Bool(true)))
}

/// Parses `#f`.
fn false_p(s: &str) -> ParserResult<LispVal> {
    string_p("#f", s).map(|(tail, _)| (tail, LispVal::Bool(false)))
}

/// Parses [LispVal::Bool].
fn lisp_bool(s: &str) -> ParserResult<LispVal> {
    alt! { s ; false_p | true_p }
}

// Numbers.

/// Parses a digit as a [&str].
fn digit_p_as_str(s: &str) -> ParserResult<&str> {
    alt_string_p! {
        s ; zero "0" | one "1" | two "2" | three "3" | four "4"
        | five "5" | six "6" | seven "7" | eight "8" | nine "9"
    }
}

/// Convert [&str] to `A`. Can throw errors.
fn str_to<A: core::str::FromStr>(s: &str) -> A {
    s.parse::<A>().ok().unwrap()
}

fn snd_to_str<'a>(res: UnwrappedParserResult<'a, &str>) -> UnwrappedParserResult<'a, usize> {
    (res.0, str_to::<usize>(res.1))
}

/// Parses a digit as a [usize].
fn digit_p(s: &str) -> ParserResult<usize> {
    digit_p_as_str(s).map(snd_to_str)
}

/// Concatenate `digit` and `tail`.
/// `digit` is the most significant digit of the result.
fn combine_positive_numbers(digit: usize, tail: usize) -> usize {
    str_to::<usize>(&*format!("{}{}", digit, tail))
}

/// Parses a positive number.
fn positive_number_p(s: &str) -> ParserResult<usize> {
    repeat::<usize>(&digit_p, s, &combine_positive_numbers)
}

// TODO
/// Combine all of the parsers into one to parse a [LispVal].
fn parse(s: &str) -> ParserResult<LispVal> {
    alt! { s ; lisp_nil | lisp_nil | lisp_bool }
}

/* TODO
documentation
isize
numbers
test macro
macro parser_combinator!
    * repeat
    | alternative
    & and
    ()
*/

/* TODO
Comments

fn lisp_quote(s: &str) -> ParserResult<LispVal> {
}
fn lisp_s_expr(s: &str) -> ParserResult<LispVal> {
}
fn lisp_string(s: &str) -> ParserResult<LispVal> {
}
fn lisp_atom(s: &str) -> ParserResult<LispVal> {
}
fn lisp_number(s: &str) -> ParserResult<LispVal> {
}
fn lisp_lisp(s: &str) -> ParserResult<LispVal> {
}
fn lisp_lambda(s: &str) -> ParserResult<LispVal> {
}
fn lisp_fun(s: &str) -> ParserResult<LispVal> {
}
*/
