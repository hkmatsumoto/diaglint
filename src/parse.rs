use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::{map, value},
    multi::many1,
    sequence::delimited,
    IResult,
};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone)]
pub struct Spanned<'a, T> {
    inner: T,
    span: Span<'a>,
}

impl<'a, T> Spanned<'a, T> {
    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn span(&self) -> Span<'a> {
        self.span
    }
}

pub type Sentence<'a> = Vec<Spanned<'a, Element>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Normal(String),
    /// Words surrounded by \`\`
    Placeholder(String),
    /// `...`
    Ellipsis,
    /// `.`
    Period,
    /// `:`
    Colon,
    /// `;`
    Semicolon,
}

impl Element {
    pub fn len(&self) -> usize {
        use Element::*;
        match self {
            Normal(s) | Placeholder(s) => s.len(),
            Ellipsis => 3,
            Period | Colon | Semicolon => 1,
        }
    }
}

pub fn parse_message(message: &str) -> IResult<Span, Vec<Sentence>> {
    let message = Span::new(message);
    many1(parse_element)(message).map(|(m, sentences)| {
        let sentences = sentences
            .split_inclusive(|elem| matches!(elem.inner, Element::Period))
            .map(|s| s.to_vec())
            .collect::<Vec<Sentence>>();
        (m, sentences)
    })
}

fn parse_element(s: Span) -> IResult<Span, Spanned<Element>> {
    use Element::*;

    let (s, pos) = position(s)?;
    let (s, element) = alt((
        map(is_not("`.:;"), |s: Span| Normal(s.fragment().to_string())),
        map(delimited(char('`'), is_not("`"), char('`')), |s: Span| {
            Placeholder(s.fragment().to_string())
        }),
        value(Ellipsis, tag("...")),
        value(Period, char('.')),
        value(Colon, char(':')),
        value(Semicolon, char(';')),
    ))(s)?;

    let spanned = Spanned {
        inner: element,
        span: pos,
    };
    Ok((s, spanned))
}
