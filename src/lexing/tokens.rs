use crate::common::multiphase::{
    Accessibility, Identifier, InterpolatedString, OverloadableInfixOperator, PostfixOperator,
    PseudoIdentifier, Shebang, SyDoc, SylanString,
};
use crate::common::version::Version;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Literal {
    Char(char),
    InterpolatedString(InterpolatedString),
    String(SylanString),

    // TODO: reimplement using a variable-width numerics library, like GMP but not GPL licenced.
    Number(i64, u64),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BranchingAndJumping {
    If,
    Else,
    While,
    For,
    Switch,
    Select,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum DeclarationHead {
    Class,
    Extend,
    Fun,
    Interface,
    Module,
    Import,
    Package,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Grouping {
    CloseBrace,
    CloseParentheses,
    CloseSquareBracket,
    OpenBrace,
    OpenParentheses,
    OpenSquareBracket,
}

/// Unlike other languages, modifiers always come _after_ declaration heads.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Modifier {
    Accessibility(Accessibility),

    Embed,

    // Extern means quite different things depending on whether it refers to a
    // bindings, functions, a type. For bindings and functions, it means they
    // are defined in another compiled artefact, perhaps written in a different
    // language. For types, it means that it is defined by Sylan itself. Types
    // cannot be defined outside of Sylan, only the _operations_ and _runtime
    // values_ of said types.
    Extern,

    Ignorable,
    Operator,
    Override,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ModuleDefinitions {
    Exports,
    Reject,
    Requires,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Binding {
    As,
    Assign,
    Final,
    Var,
}

/// All tokens that can currently exist in all version of a Sylan program source.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Token {
    Identifier(Identifier),
    Literal(Literal),
    Shebang(Shebang),
    SyDoc(SyDoc),
    Version(Version),

    Binding(Binding),
    BranchingAndJumping(BranchingAndJumping),
    DeclarationHead(DeclarationHead),
    Grouping(Grouping),
    Modifier(Modifier),
    ModuleDefinitions(ModuleDefinitions),
    OverloadableInfixOperator(OverloadableInfixOperator),
    PostfixOperator(PostfixOperator),
    PseudoIdentifier(PseudoIdentifier),

    Colon,
    Dot,
    Eof,
    LambdaArrow,
    Rest,
    SubItemSeparator,
    Throw,
    Timeout,
    Using,

    /// Does nothing but reserves keywords for future use.
    ReservedKeyword,

    With,
}

/// EOF is a special type of token because it simplifies logic over handling it
/// in a special typed manner in every lexing case.
impl Default for Token {
    fn default() -> Token {
        Token::Eof
    }
}
