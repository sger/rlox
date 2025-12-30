use crate::token::{Literal, Token};
use crate::token_type::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advanced();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let t = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t);
            }
            '=' => {
                let t = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t);
            }
            '<' => {
                let t = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t);
            }
            '>' => {
                let t = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t);
            }

            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advanced();
                    }
                } else if self.matches('*') {
                    self.block_comment(true);
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {
                // Ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }

            '"' => self.string(),

            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    self.error_at_line("Unexpected character.");
                }
            }
        }
    }

    fn block_comment(&mut self, allow_nesting: bool) {
        let mut depth: usize = 1;

        while depth > 0 {
            if self.is_at_end() {
                self.error_at_line("Unterminated block comment.");
                return;
            }

            let c = self.peek();

            if c == '\n' {
                self.line += 1;
                self.advanced();
                continue;
            }

            if c == '/' && self.peek_next() == '*' {
                if allow_nesting {
                    self.advanced();
                    self.advanced();
                    depth += 1;
                    continue;
                } else {
                    self.advanced();
                    continue;
                }
            }

            if c == '*' && self.peek_next() == '/' {
                self.advanced();
                self.advanced();
                depth -= 1;
                continue;
            }

            self.advanced();
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advanced();
        }

        let text = self.lexeme();
        let token_type = keyword_type(&text).unwrap_or(TokenType::Identifier);
        self.add_token(token_type);
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advanced();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advanced();

            while is_digit(self.peek()) {
                self.advanced();
            }
        }

        let text = self.lexeme();

        let value: f64 = text.parse().unwrap_or_else(|_| {
            self.error_at_line("Invalid number literal.");
            0.0
        });

        self.add_token_literal(TokenType::Number, Literal::Number(value));
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        let mut it = self.source[self.current..].chars();
        let _first = it.next();
        it.next().unwrap_or('\0')
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advanced();
        }

        if self.is_at_end() {
            self.error_at_line("Unterminated string.");
            return;
        }

        self.advanced();

        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_literal(TokenType::String, Literal::String(value));
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.add_token_opt_literal(token_type, Some(literal));
    }

    fn error_at_line(&self, message: &str) {
        eprintln!("[line {}] Error: {}", self.line, message);
    }
    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }
        self.current += expected.len_utf8();
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current..].chars().next().unwrap_or('\0')
        }
    }

    fn advanced(&mut self) -> char {
        let c = self.source[self.current..].chars().next().unwrap_or('\0');
        self.current += c.len_utf8();
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_opt_literal(token_type, None);
    }

    fn add_token_opt_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.lexeme();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn lexeme(&self) -> String {
        self.source[self.start..self.current].to_string()
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn keyword_type(text: &str) -> Option<TokenType> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Literal;
    use crate::token_type::TokenType;

    fn scan(src: &str) -> Vec<crate::token::Token> {
        Scanner::new(src).scan_tokens()
    }

    fn token_types(src: &str) -> Vec<TokenType> {
        scan(src).into_iter().map(|t| t.token_type).collect()
    }

    #[test]
    fn empty_source_produces_only_eof() {
        assert_eq!(token_types(""), vec![TokenType::Eof]);
    }

    #[test]
    fn scans_single_character_tokens() {
        let token_types = token_types("( ) { } , . - + ; * /");
        assert_eq!(
            token_types,
            vec![
                TokenType::LeftParen,
                TokenType::RightParen,
                TokenType::LeftBrace,
                TokenType::RightBrace,
                TokenType::Comma,
                TokenType::Dot,
                TokenType::Minus,
                TokenType::Plus,
                TokenType::Semicolon,
                TokenType::Star,
                TokenType::Slash,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn scans_one_or_two_character_operators() {
        let token_types = token_types("! != = == < <= > >=");
        assert_eq!(
            token_types,
            vec![
                TokenType::Bang,
                TokenType::BangEqual,
                TokenType::Equal,
                TokenType::EqualEqual,
                TokenType::Less,
                TokenType::LessEqual,
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn ignores_whitespace() {
        let token_types = token_types(" \r\t     \t\r ");
        assert_eq!(token_types, vec![TokenType::Eof]);
    }

    #[test]
    fn newlines_increment_line_counter() {
        let tokens = scan("\n\n+");
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[0].line, 3);
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn line_comment_is_ignored_until_newline() {
        let token_types = token_types("// hello\n+");
        assert_eq!(token_types, vec![TokenType::Plus, TokenType::Eof]);
    }

    #[test]
    fn line_comment_at_eof_is_ignored() {
        let token_types = token_types("// hello");
        assert_eq!(token_types, vec![TokenType::Eof]);
    }

    #[test]
    fn scans_string_literal_and_sets_literal_value() {
        let tokens = scan("\"hello\"");
        assert_eq!(tokens[0].token_type, TokenType::String);

        match tokens[0].literal.as_ref() {
            Some(Literal::String(value)) => assert_eq!(value, "hello"),
            other => panic!("expected Literal::String(\"hello\"), got {:?}", other),
        }

        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn string_literal_can_contain_newlines_and_updates_line() {
        let tokens = scan("\"a\nb\"");
        assert_eq!(tokens[0].token_type, TokenType::String);
        assert_eq!(tokens[0].line, 2);

        match tokens[0].literal.as_ref() {
            Some(Literal::String(value)) => assert_eq!(value, "a\nb"),
            other => panic!("expected Literal::String(\"a\\nb\"), got {:?}", other),
        }
    }

    #[test]
    fn scans_integer_number_literal() {
        let tokens = scan("123");
        assert_eq!(tokens[0].token_type, TokenType::Number);

        match tokens[0].literal.as_ref() {
            Some(Literal::Number(n)) => assert_eq!(*n, 123.0),
            other => panic!("expected Literal::Number(123.0), got {:?}", other),
        }

        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn scans_fractional_number_literal() {
        let tokens = scan("123.456");
        assert_eq!(tokens[0].token_type, TokenType::Number);

        match tokens[0].literal.as_ref() {
            Some(Literal::Number(n)) => assert!((*n - 123.456).abs() < 1e-12),
            other => panic!("expected Literal::Number(123.456), got {:?}", other),
        }

        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn dot_is_not_fractional_part_without_trailing_digit() {
        let token_types = token_types("123.");
        assert_eq!(
            token_types,
            vec![TokenType::Number, TokenType::Dot, TokenType::Eof]
        );
    }

    #[test]
    fn scans_identifier() {
        let tokens = scan("foo_bar");
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "foo_bar");
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn identifiers_can_contain_digits_after_first_character() {
        let tokens = scan("foo123");
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "foo123");
    }

    #[test]
    fn recognizes_keywords() {
        let token_types = token_types(
            "and class else false for fun if nil or print return super this true var while",
        );

        assert_eq!(
            token_types,
            vec![
                TokenType::And,
                TokenType::Class,
                TokenType::Else,
                TokenType::False,
                TokenType::For,
                TokenType::Fun,
                TokenType::If,
                TokenType::Nil,
                TokenType::Or,
                TokenType::Print,
                TokenType::Return,
                TokenType::Super,
                TokenType::This,
                TokenType::True,
                TokenType::Var,
                TokenType::While,
                TokenType::Eof,
            ]
        )
    }

    #[test]
    fn keywords_are_not_prefixes_of_identifiers() {
        let tokens = scan("class classy");
        assert_eq!(tokens[0].token_type, TokenType::Class);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "classy");
        assert_eq!(tokens[2].token_type, TokenType::Eof);
    }

    #[test]
    fn scans_mixed_program_snippet() {
        let token_types = token_types("var a = 1; print a;");
        assert_eq!(
            token_types,
            vec![
                TokenType::Var,
                TokenType::Identifier,
                TokenType::Equal,
                TokenType::Number,
                TokenType::Semicolon,
                TokenType::Print,
                TokenType::Identifier,
                TokenType::Semicolon,
                TokenType::Eof,
            ]
        );
    }

    #[test]
    fn block_comment_is_ignored() {
        let token_types = token_types("1 /* comment */ 2");
        assert_eq!(
            token_types,
            vec![TokenType::Number, TokenType::Number, TokenType::Eof]
        );
    }

    #[test]
    fn block_comment_counts_newlines() {
        let tokens = scan("/* a\nb\nc */ +");
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[0].line, 3);
    }

    #[test]
    fn nested_block_comments_are_ignored() {
        let token_types = token_types("1 /* outer /* inner */ outer */ 2");
        assert_eq!(
            token_types,
            vec![TokenType::Number, TokenType::Number, TokenType::Eof]
        );
    }
}
