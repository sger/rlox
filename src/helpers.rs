use crate::token_type::TokenType;

#[inline]
pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

#[inline]
pub fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[inline]
pub fn is_alpha_numeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

pub fn keyword_type(text: &str) -> Option<TokenType> {
    Some(match text {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => return None,
    })
}
