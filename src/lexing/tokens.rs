#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Boolean(bool),
    Char(char),
    Identifier(String),
    InterpolatedString(String),
    Number(i64, u64),
    Shebang(String),
    String(String),
    SyDoc(String),
    Version(u64, u64),
    Add,
    And,
    Assign,
    Bind,
    BitwiseAnd,
    BitwiseNot,
    BitwiseOr,
    BitwiseXor,
    Case,
    Class,
    CloseBrace,
    CloseParentheses,
    CloseSquareBracket,
    Colon,
    Compose,
    Continue,
    Default,
    Divide,
    Do,
    Dot,
    Else,
    Eof,
    Equals,
    Extends,
    For,
    Get,
    GreaterThan,
    GreaterThanOrEquals,
    If,
    Ignore,
    Implements,
    Import,
    Interface,
    LambdaArrow,
    LessThan,
    LessThanOrEquals,
    MethodHandle,
    Modulo,
    Multiply,
    Not,
    NotEquals,
    OpenBrace,
    OpenParentheses,
    OpenSquareBracket,
    Or,
    Override,
    Package,
    Pipe,
    Public,
    Select,
    ShiftLeft,
    ShiftRight,
    SubItemSeparator,
    Subtract,
    Super,
    Switch,
    Throw,
    Timeout,
    Var,
}

impl Default for Token {
    fn default() -> Token {
        Token::Eof
    }
}
