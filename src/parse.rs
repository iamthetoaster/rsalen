// use std::collections::HashMap;

use logos::Logos;

// static mut SYMBOL_TABLE: Option<HashMap<String, String>> = None;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("procedure")]
    Procedure,
    
    #[regex("(_[a-zA-Z0-9_\\-]*)|([a-zA-Z][a-zA-Z0-9_\\-]*)")]
    Identifier,
    
    #[token("{")]
    OpenCurly,
    
    #[token("}")]
    CloseCurly,

    #[token(";")]
    Semicolon,
    
    #[token("?")]
    QuestionMark,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    // #[token("not")]
    // Not,
    
    // #[token("and")]
    // And,
    
    // #[token("or")]
    // Or,
    
    // #[token("(")]
    // OpenParen,
    
    // #[token(")")]
    // CloseParen,

    // #[token("if")]
    // If,

    // #[token("while")]
    // While,

    // #[token("else")]
    // Else,

    #[error]
    Error,
}

