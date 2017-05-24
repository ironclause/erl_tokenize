//! Tokens.
use std::ops::Deref;
use num::BigUint;

/// Character token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Char(pub char);
impl Deref for Char {
    type Target = char;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Variable token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Var(pub String);
impl Deref for Var {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// String token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Str(pub String);
impl Deref for Str {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Atom token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Atom(pub String);
impl Deref for Atom {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Comment token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Comment(pub String);
impl Deref for Comment {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// White space token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Whitespace {
    /// `' '`
    Space,

    /// `'\t'`
    Tab,

    /// `'\r'`
    Return,

    /// `'\n'`
    Newline,

    /// `'\u{A0}'`
    NoBreakSpace,
}
impl Whitespace {
    /// Coverts to the corresponding character.
    pub fn as_char(&self) -> char {
        match *self {
            Whitespace::Space => ' ',
            Whitespace::Tab => '\t',
            Whitespace::Return => '\r',
            Whitespace::Newline => '\n',
            Whitespace::NoBreakSpace => '\u{A0}',
        }
    }
}

/// Integer token.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Int(pub BigUint);
impl Deref for Int {
    type Target = BigUint;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Floating point number token.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Float(pub f64);
impl Deref for Float {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Symbol token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Symbol {
    /// `[`
    OpenSquare,

    /// `]`
    CloseSquare,

    /// `(`
    OpenParen,

    /// `)`
    CloseParen,

    /// `{`
    OpenBrace,

    /// `}`
    CloseBrace,

    /// `#`
    Sharp,

    /// `/`
    Slash,

    /// ``
    Dot,

    /// `,`
    Comma,

    /// `:`
    Colon,

    /// `;`
    Semicolon,

    /// `=`
    Match,

    /// `:=`
    MapMatch,

    /// `|`
    VerticalBar,

    /// `||`
    DoubleVerticalBar,

    /// `?`
    Question,

    /// `!`
    Not,

    /// `-`
    Hyphen,

    /// `--`
    MinusMinus,

    /// `+`
    Plus,

    /// `++`
    PlusPlus,

    /// `*`
    Multiply,

    /// `->`
    RightAllow,

    /// `<-`
    LeftAllow,

    /// `=>`
    DoubleRightAllow,

    /// `<=`
    DoubleLeftAllow,

    /// `>>`
    DoubleRightAngle,

    /// `<<`
    DoubleLeftAngle,

    /// `==`
    Eq,

    /// `=:=`
    ExactEq,

    /// `/=`
    NotEq,

    /// `=/=`
    ExactNotEq,

    /// `>`
    Greater,

    /// `>=`
    GreaterEq,

    /// `<`
    Less,

    /// `=<`
    LessEq,
}

/// Keyword tokens.
///
/// Reference: [Erlang's Reserved Words][Reserved Words]
///
/// [Reserved Words]: http://erlang.org/doc/reference_manual/introduction.html#id61721
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    /// `after`
    After,

    /// `and`.
    And,

    /// `andalso`.
    Andalso,

    /// `band`.
    Band,

    /// `begin`.
    Begin,

    /// `bnot`.
    Bnot,

    /// `bor`.
    Bor,

    /// `bsl`.
    Bsl,

    /// `bsr`.
    Bsr,

    /// `bxor`.
    Bxor,

    /// `case`.
    Case,

    /// `catch`.
    Catch,

    /// `cond`.
    Cond,

    /// `div`.
    Div,

    /// `end`.
    End,

    /// `fun`.
    Fun,

    /// `if`.
    If,

    /// `let`.
    Let,

    /// `not`.
    Not,

    /// `of`.
    Of,

    /// `or`.
    Or,

    /// `orelse`.
    Orelse,

    /// `receive`.
    Receive,

    /// `rem`.
    Rem,

    /// `try`.
    Try,

    /// `when`.
    When,

    /// `xor`.
    Xor,
}
impl Keyword {
    /// Tries to convert from a string to a `Keyword` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use erl_tokenize::tokens::Keyword;
    ///
    /// assert_eq!(Keyword::from_str("bor"), Some(Keyword::Bor));
    /// assert_eq!(Keyword::from_str("foo"), None);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "after" => Some(Keyword::After),
            "and" => Some(Keyword::And),
            "andalso" => Some(Keyword::Andalso),
            "band" => Some(Keyword::Band),
            "begin" => Some(Keyword::Begin),
            "bnot" => Some(Keyword::Bnot),
            "bor" => Some(Keyword::Bor),
            "bsl" => Some(Keyword::Bsl),
            "bsr" => Some(Keyword::Bsr),
            "bxor" => Some(Keyword::Bxor),
            "case" => Some(Keyword::Case),
            "catch" => Some(Keyword::Catch),
            "cond" => Some(Keyword::Cond),
            "div" => Some(Keyword::Div),
            "end" => Some(Keyword::End),
            "fun" => Some(Keyword::Fun),
            "if" => Some(Keyword::If),
            "let" => Some(Keyword::Let),
            "not" => Some(Keyword::Not),
            "of" => Some(Keyword::Of),
            "or" => Some(Keyword::Or),
            "orelse" => Some(Keyword::Orelse),
            "receive" => Some(Keyword::Receive),
            "rem" => Some(Keyword::Rem),
            "try" => Some(Keyword::Try),
            "when" => Some(Keyword::When),
            "xor" => Some(Keyword::Xor),
            _ => None,
        }
    }

    /// Returns the string representation of this keyword.
    pub fn as_str(&self) -> &'static str {
        match *self {
            Keyword::After => "after",
            Keyword::And => "and",
            Keyword::Andalso => "andalso",
            Keyword::Band => "band",
            Keyword::Begin => "begin",
            Keyword::Bnot => "bnot",
            Keyword::Bor => "bor",
            Keyword::Bsl => "bsl",
            Keyword::Bsr => "bsr",
            Keyword::Bxor => "bxor",
            Keyword::Case => "case",
            Keyword::Catch => "catch",
            Keyword::Cond => "cond",
            Keyword::Div => "div",
            Keyword::End => "end",
            Keyword::Fun => "fun",
            Keyword::If => "if",
            Keyword::Let => "let",
            Keyword::Not => "not",
            Keyword::Of => "of",
            Keyword::Or => "or",
            Keyword::Orelse => "orelse",
            Keyword::Receive => "receive",
            Keyword::Rem => "rem",
            Keyword::Try => "try",
            Keyword::When => "when",
            Keyword::Xor => "xor",
        }
    }
}
