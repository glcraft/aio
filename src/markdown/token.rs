use smartstring::alias::String;

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
pub enum Token {
    Text(String),
    Newline,
    InlineStyle(Marker),
    BeginCode{
        language: Option<String>,
    },
    EndCode,
    Heading(u8),
    ListItem(u8),
}