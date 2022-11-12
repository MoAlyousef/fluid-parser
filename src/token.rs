#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    #[default]
    Unknown,
    Word,
    OpenBrace,
    CloseBrace,
    Eof,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub word: &'a str,
    pub start: usize,
    pub end: usize,
}
