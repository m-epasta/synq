use crate::error::ParseError;
use crate::synq::scanner::Scanner;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Attribute,
    // NOTE: Comments are not kept by the lexer, they are skipped by the skip function.
    // You may want a lossless syntax tree (LST) for a future LSP

    // // Mono means // and multi means /* */
    // MonoComment,
    // MultiComment,
    Const,
    Enum,
    Eof,
    Equal,
    Frame,
    // An int value, it is injected at comptime or can be defined manually
    // NOTE: Missing indexes are resolved from high to bottom (See examples)
    Index,
    Import,
    Ident,
    // Equivalent of rpc in proto
    Message,
    Newline,
    Option,
    Package,
    // Equivalent of service in proto
    Photon,
    Quote,
    Use,

    Lbrace,
    Rbrace,
    Lparen,
    Rparen,

    Dot,
    Greater,
    Less,
    GreaterThan,
    LessThan,
    Star,
    Slash,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub typ: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
    pub col: usize,
}

impl<'a> Scanner<'a> {
    pub fn next_token(&mut self) -> Result<Token<'a>, ParseError> {
        self.skip();

        let (line, col) = self.curr_loc();
        let start = self.current_pos();

        let Some(c) = self.current() else {
            return Ok(Token {
                typ: TokenType::Eof,
                lexeme: "",
                line,
                col,
            });
        };

        if c == b'\n' {
            let typ = TokenType::Newline;
            let (nl, ncol) = self.curr_loc();
            self.bump_nl();
            return Ok(Token {
                typ,
                lexeme: "\n",
                line: nl,
                col: ncol,
            });
        }

        if c.is_ascii_digit() {
            while let Some(b) = self.current() {
                // NOTE: For performance, the lexer never goes backward,
                // It also means that a 1.2.3 can be parsed as a digit/index
                // Instead, this check will happen at semantic analyzing part
                if b.is_ascii_digit() || b == b'.' || b == b'_' {
                    self.bump();
                } else {
                    break;
                }
            }
            return Ok(Token {
                typ: TokenType::Index,
                lexeme: self.slice(start, self.current_pos()),
                line,
                col,
            });
        }

        if c.is_ascii_alphabetic() || c == b'_' {
            while let Some(b) = self.current() {
                if b.is_ascii_alphanumeric() || b == b'_' {
                    self.bump();
                } else {
                    break;
                }
            }

            let lexeme = self.slice(start, self.current_pos());
            let typ = match lexeme {
                "package" => TokenType::Package,
                "import" => TokenType::Import,
                "const" => TokenType::Const,
                "option" => TokenType::Option,
                "enum" => TokenType::Enum,
                "frame" => TokenType::Frame,
                "synq" => TokenType::Photon,
                "message" => TokenType::Message,
                "use" => TokenType::Use,
                _ => TokenType::Ident,
            };

            return Ok(Token {
                typ,
                lexeme,
                line,
                col,
            });
        }

        self.bump();
        let typ = match c {
            b'@' => TokenType::Attribute,
            b'=' => TokenType::Equal,
            b'{' => TokenType::Lbrace,
            b'}' => TokenType::Rbrace,
            b'(' => TokenType::Lparen,
            b')' => TokenType::Rparen,
            b'*' => TokenType::Star,
            b'.' => TokenType::Dot,
            b'>' if self.next_token().unwrap().typ == TokenType::Equal => TokenType::GreaterThan,
            b'>' => TokenType::Greater,
            b'<' if self.next_token().unwrap().typ == TokenType::Equal => TokenType::LessThan,
            b'<' => TokenType::Less,
            b'/' => TokenType::Slash,
            b'"' => TokenType::Quote,
            _ => {
                return Err(ParseError::Unexpected {
                    typ: (c as char).to_string(),
                    line,
                    col,
                });
            }
        };

        Ok(Token {
            typ,
            lexeme: self.slice(start, self.current_pos()),
            line,
            col,
        })
    }
}
