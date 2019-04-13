use internal::{Err, IResult, ParseError};
use ::lib::std::ops::RangeFrom;
use traits::{AsChar, AtEof, FindToken, InputIter, InputLength, Slice};
use util::ErrorKind;

pub fn char<I, Error: ParseError<I>>(c: char) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + AtEof,
  <I as InputIter>::Item: AsChar,
{
  move |i: I| match (i).iter_elements().next().map(|t| {
    let b = t.as_char() == c;
    (&c, b)
  }) {
    Some((c, true)) => Ok((i.slice(c.len()..), c.as_char())),
    _ => Err(Err::Error(Error::from_char(i, c))),
  }
}


pub fn one_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + AtEof,
  <I as InputIter>::Item: AsChar + Copy,
  T: FindToken<<I as InputIter>::Item>,
{
  move |i: I| match (i).iter_elements().next().map(|c| (c, list.find_token(c))) {
    Some((c, true)) => Ok((i.slice(c.len()..), c.as_char())),
    _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::OneOf))),
  }
}

pub fn none_of<I, T, Error: ParseError<I>>(list: T) -> impl Fn(I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + AtEof,
  <I as InputIter>::Item: AsChar + Copy,
  T: FindToken<<I as InputIter>::Item>,
{
  move |i: I| match (i).iter_elements().next().map(|c| (c, !list.find_token(c))) {
    Some((c, true)) => Ok((i.slice(c.len()..), c.as_char())),
    _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::NoneOf))),
  }
}

/// matches a newline character '\\n'
pub fn newline<I, Error: ParseError<I>>(input: I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + AtEof,
  <I as InputIter>::Item: AsChar,
{
  char('\n')(input)
}

/// matches a tab character '\t'
pub fn tab<I, Error: ParseError<I>>(input: I) -> IResult<I, char, Error>
where
  I: Slice<RangeFrom<usize>> + InputIter + AtEof,
  <I as InputIter>::Item: AsChar,
{
  char('\t')(input)
}

/// matches one byte as a character. Note that the input type will
/// accept a `str`, but not a `&[u8]`, unlike many other nom parsers.
///
/// # Example
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::{character::complete::anychar, ErrorKind};
/// # fn main() {
/// assert_eq!(anychar::<_,(&str, ErrorKind)>("abc"), Ok(("bc",'a')));
/// # }
/// ```
pub fn anychar<T, E: ParseError<T>>(input: T) -> IResult<T, char, E>
where
  T: InputIter + InputLength + Slice<RangeFrom<usize>> + AtEof,
  <T as InputIter>::Item: AsChar,
{
  let mut it = input.iter_indices();
  match it.next() {
    None => Err(Err::Error(E::from_error_kind(input, ErrorKind::Eof))),
    Some((_, c)) => match it.next() {
      None => Ok((input.slice(input.input_len()..), c.as_char())),
      Some((idx, _)) => Ok((input.slice(idx..), c.as_char())),
    },
  }
}