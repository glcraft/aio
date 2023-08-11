// use smartstring::alias::String;

#[derive(Debug, PartialEq, Clone)]
pub enum InlineStyleToken {
    OneStar,
    TwoStars,
    ThreeStars,
    OneDash,
    TwoDashes,
    OneQuote,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Marker {
    Begin(InlineStyleToken),
    End(InlineStyleToken),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Level(usize);

impl<UInt: num_traits::Unsigned + Into<usize>> From<UInt> for Level {
    fn from(n: UInt) -> Self {
        Self(n.into())
    }
}
impl From<Level> for usize {
    fn from(n: Level) -> Self {
        n.0
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Text(String),
    Newline,
    InlineStyle(Marker),
    BeginCode,
    EndCode,
    Line,
    Heading(Level),
    ListItem(Level),
    EndDocument,
}