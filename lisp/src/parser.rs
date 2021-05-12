extern crate alloc;

use alloc::collections::btree_map::BTreeMap;
use alloc::fmt;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use alloc::borrow::ToOwned;

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

/// Parser combiner: uses the parsers from left to right.
macro_rules! then {
    ($s: expr ; $l: expr) => {
        $l($s)
    };

    ($s: expr ; $l: expr => $( $tail_p: expr)=>+ ) => {
        $l($s).and_then(|(tail, _)| then! { tail ; $( $tail_p )=>+ })
    };
}

/// Repeats a parser as much as possible and folds the results with `combine`.
fn repeat<'a, A>(p: Parser<'a, A>, s: &'a str, combine: &dyn Fn(A, A) -> A) -> ParserResult<'a, A> {
    p(s).and_then(
        |actual_res: UnwrappedParserResult<A>| match repeat(p, actual_res.0, combine) {
            None => Some(actual_res),
            x => x.map(|tail_res: UnwrappedParserResult<A>| {
                (tail_res.0, combine(actual_res.1, tail_res.1))
            }),
        },
    )
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
    s.strip_prefix(s_p).map(|tail| (tail, s_p))
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

fn snd_str_to<'a, A: core::str::FromStr>(
    res: UnwrappedParserResult<'a, &str>,
) -> UnwrappedParserResult<'a, A> {
    (res.0, str_to::<A>(res.1))
}

/// Parses a digit as a [isize].
fn digit_p(s: &str) -> ParserResult<isize> {
    digit_p_as_str(s).map(snd_str_to::<isize>)
}

/// Concatenate `digit` and `tail`.
/// `digit` is the most significant digit of the result.
fn combine_positive_numbers(digit: isize, tail: isize) -> isize {
    str_to::<isize>(&*format!("{}{}", digit, tail))
}

/// Parses a positive number.
fn positive_number_p(s: &str) -> ParserResult<isize> {
    repeat::<isize>(&digit_p, s, &combine_positive_numbers)
}

/// Parses a negative number.
fn negative_number_p(s: &str) -> ParserResult<isize> {
    let minus_p = |s| string_p("-", s);
    (then! { s ; minus_p => positive_number_p }).map(|(tail, n)| (tail, -n))
}

/// Parses [LispVal::Number].
// TODO overflow
fn lisp_number(s: &str) -> ParserResult<LispVal> {
    (alt! {s ; positive_number_p | negative_number_p }).map(|(tail, n)| (tail, LispVal::Number(n)))
}

fn is_letter(c: char) -> bool {
    matches!(c, 'a'..='z') || matches!(c, 'A'..='Z')
}

/// Parses a string of letters as a `&str`.
/// The parsed string can be empty.
fn letter_string_p(s: &str) -> ParserResult<&str> {
    if s.chars().all(is_letter) {
        Some(("", s))
    } else {
        let end = (s.find(|c| !is_letter(c))).unwrap();
        Some((&s[end..], &s[0..end - 1]))
    }
}

fn lisp_string(s: &str) -> ParserResult<LispVal> {
    let quote_p = |s| string_p(r#"""#, s);
    (then! { s ; quote_p => letter_string_p => quote_p })
        .map(|(tail, string)| (tail, LispVal::String(string)))
}

fn lisp_atom(s: &str) -> ParserResult<LispVal> {
    letter_string_p(s).and_then(|(tail, name)| {
        if name == "" {
            None
        } else {
            Some((tail, LispVal::Atom(name)))
        }
    })
}

// TODO
/// Combine all of the parsers into one to parse a [LispVal].
fn parse(s: &str) -> ParserResult<LispVal> {
    alt! { s ; lisp_bool | lisp_number | lisp_string | lisp_atom | lisp_nil }
}

/* TODO
documentation
test macro

lambda
list
+
-
*
/
==
<
quote
if
let
begin
define
not
or
and
print
concat strings

comments

macro parser_combinator!
    * repeat
    +
    | alternative
    => and
    () list
*/
