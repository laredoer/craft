#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT(String),  // add, foobar, x, y, ...
    INT(String),    // 12345
    STRING(String), // "abc"

    ASSIGN, // "="
    COMMA,  // ","
    SHAP,   // "#"

    BANG,  // "!"
    MINUS, // "-"
    // "("
    LPAREN,
    RPAREN, // ")"

    LBRACKET,  // "["
    RBRACKET,  // "]"
    SEMICOLON, // ";"

    TRUE,  // "true"
    FALSE, // "false"
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "true" => Token::TRUE,
        "false" => Token::FALSE,
        _ => Token::IDENT(ident.to_string()),
    }
}
