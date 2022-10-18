/// Enum for tokens
#[allow(unused)] // TODO: Use all of them!
#[derive(Debug)]
pub enum Token {
    // Keywords
    Class,
    Public,
    Private,
    Inherits,
    New,
    EndClass,

    Function,
    ByVal, // annotations in function definition. byval is assumed if none given.
    ByRef, // e.g. function foobar(x: byval, y: byref)
    Return,
    EndFunction,

    Procedure,
    EndProcedure,

    If,
    Then,
    ElseIf,
    Else,
    EndIf,

    Switch,
    Case,
    EndSwitch,

    For,
    To,
    Next,

    While,
    EndWhile,

    Do,
    Until,

    Global,
    Array,

    // Parens
    LParen,
    RParen,

    LSquareParen,
    RSquareParen,

    // Logical operators
    And,
    Or,
    Not,

    EqTo,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    Plus,
    Minus,
    Mul,
    Div,      // regular division
    Mod,      // Mod
    FloorDiv, // DIV
    Exp,      // ^

    // Hold values
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),

    // Misc
    Colon,
    Dot,
    Comma,
    AssignEq,
}
