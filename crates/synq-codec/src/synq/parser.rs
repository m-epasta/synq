use crate::{
    error::ParseError,
    synq::lexer::{Token, TokenType},
};
use std::mem;

static FHINTSTABLE: [&str; 3] = ["repeated", "optional", "oneof"];

#[derive(Debug, Default)]
pub struct Ast {
    pub package: String,
    pub import: Vec<String>,
    pub options: Vec<Hint>,
    pub consts: Vec<ConstDecl>,
    pub enums: Vec<EnumDecl>,
    pub frames: Vec<FrameDecl>,
    pub synqs: Vec<PhotonDecl>,
}

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub val: String,
}

#[derive(Debug)]
pub struct ConstDecl {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Hint {
    pub name: String,
    pub hint: String,
}

#[derive(Debug)]
pub struct EnumDecl {
    pub name: String,
    pub consts: Vec<ConstDecl>,
}

#[derive(Debug)]
pub struct FrameDecl {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub fields: Vec<FieldDecl>,
    pub reservations: Vec<ReservedFieldDecl>,
}

#[derive(Debug)]
pub struct FieldDecl {
    pub hint: Option<String>,
    pub typ: String,
    pub name: String,
    pub reservations: Option<ReservedFieldDecl>,
}

#[derive(Debug)]
pub struct ReservedFieldDecl {
    pub reservations: Vec<String>,
}

#[derive(Debug)]
pub struct PhotonDecl {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub messages: Vec<MessageDecl>,
}

#[derive(Debug)]
pub struct MessageDecl {
    pub name: String,
    pub req: String,
    pub res: String,
    pub body: Option<Opt>,
}

// TODO: Add support for options
#[allow(dead_code)]
#[derive(Debug)]
pub struct Opt {
    pub name: String,
    pub body: Vec<String>,
}

pub(crate) struct Parser<'tokens, 'a> {
    tokens: &'tokens Vec<Token<'a>>,
    pos: usize,
}

impl<'tokens, 'a> Parser<'tokens, 'a> {
    #[inline]
    pub fn new(tokens: &'tokens Vec<Token<'a>>) -> Self {
        Self { tokens, pos: 0 }
    }

    #[inline]
    fn peek(&self) -> Option<Token<'a>> {
        self.tokens.get(self.pos).cloned()
    }

    #[inline]
    fn advance(&mut self) -> Option<Token<'a>> {
        let tok = self.peek();
        self.pos += 1;
        tok
    }

    #[inline]
    fn peek_type(&self) -> Option<TokenType> {
        self.peek().map(|t| t.typ)
    }

    pub(crate) fn parse(&mut self) -> Result<Ast, ParseError> {
        let mut ast = Ast::default();
        let mut pending_attrs = Vec::new();

        while let Some(tok) = self.peek() {
            if tok.typ == TokenType::Eof {
                break;
            }

            match tok.typ {
                TokenType::Package => {
                    self.advance();
                    let pkg = self.advance().unwrap().lexeme.to_string();
                    ast.package.push_str(&pkg);
                }

                TokenType::Import => {
                    self.advance();
                    let pkg = self.advance().unwrap().lexeme.to_string();
                    ast.import.push(pkg);
                }

                TokenType::Const => {
                    self.advance();
                    let name = self.advance().unwrap().lexeme.to_string();
                    self.advance();
                    let value = self.advance().unwrap().lexeme.to_string();
                    ast.consts.push(ConstDecl { name, value });
                }

                TokenType::Option => {
                    self.advance();
                    let name = self.advance().unwrap().lexeme.to_string();
                    self.advance();
                    let hint = self.advance().unwrap().lexeme.to_string();
                    ast.options.push(Hint { name, hint });
                }

                TokenType::Attribute => {
                    self.advance();
                    let key = self.advance().ok_or(ParseError::Unexpected {
                        typ: "EOF".into(),
                        line: tok.line,
                        col: tok.col,
                    })?;
                    let mut val = String::new();
                    if self.peek_type() == Some(TokenType::Ident) {
                        val = self.advance().unwrap().lexeme.to_string();
                    }
                    pending_attrs.push(Attribute {
                        key: key.lexeme.to_string(),
                        val,
                    });
                }
                TokenType::Frame => {
                    self.advance();
                    let name = self.advance().unwrap().lexeme.to_string();

                    while self.peek_type() != Some(TokenType::Lbrace)
                        && self.peek_type() != Some(TokenType::Eof)
                    {
                        self.advance();
                    }
                    self.advance();

                    let mut fields = Vec::new();
                    let mut reserved_fields = Vec::new();
                    while self.peek_type() != Some(TokenType::Rbrace)
                        && self.peek_type() != Some(TokenType::Eof)
                    {
                        if self.peek_type() == Some(TokenType::Newline) {
                            self.advance();
                            continue;
                        }

                        let tok = self.peek().unwrap();
                        if tok.lexeme == "reserved" {
                            self.advance();
                            let mut reservations = Vec::new();
                            while self.peek_type() != Some(TokenType::Newline)
                                && self.peek_type() != Some(TokenType::Eof)
                            {
                                reservations.push(self.advance().unwrap().lexeme.to_string());
                            }
                            reserved_fields.push(ReservedFieldDecl { reservations });
                        } else {
                            let fhint_tok = if FHINTSTABLE.contains(&tok.lexeme) {
                                Some(self.advance().unwrap().lexeme.to_string())
                            } else {
                                None
                            };

                            let fname_tok = self.advance().ok_or(ParseError::Unexpected {
                                typ: "EOF".into(),
                                line: tok.line,
                                col: tok.col,
                            })?;
                            let ftype_tok = self.advance().ok_or(ParseError::Unexpected {
                                typ: "EOF".into(),
                                line: fname_tok.line,
                                col: fname_tok.col,
                            })?;

                            fields.push(FieldDecl {
                                hint: fhint_tok,
                                name: fname_tok.lexeme.to_string(),
                                typ: ftype_tok.lexeme.to_string(),
                                reservations: None,
                            });
                        }
                    }

                    ast.frames.push(FrameDecl {
                        name,
                        attrs: mem::take(&mut pending_attrs),
                        fields,
                        reservations: reserved_fields,
                    });
                }
                TokenType::Photon => {
                    self.advance();
                    let name = self.advance().unwrap().lexeme.to_string();

                    while self.peek_type() != Some(TokenType::Lbrace)
                        && self.peek_type() != Some(TokenType::Eof)
                    {
                        self.advance();
                    }
                    self.advance();

                    let mut messages = Vec::new();
                    while self.peek_type() != Some(TokenType::Rbrace)
                        && self.peek_type() != Some(TokenType::Eof)
                    {
                        if self.peek_type() == Some(TokenType::Message) {
                            self.advance();
                            let msg_name = self.advance().unwrap().lexeme.to_string();

                            self.advance();
                            let req = self.advance().unwrap().lexeme.to_string();
                            self.advance();

                            self.advance();
                            let res = self.advance().unwrap().lexeme.to_string();
                            self.advance();

                            self.advance();
                            let body = if self.peek_type() != Some(TokenType::Rbrace) {
                                self.advance();
                                let name = self.advance().unwrap().lexeme.to_string();
                                self.advance();
                                self.advance();
                                let mut opt = Vec::with_capacity(512);
                                while self.peek_type() != Some(TokenType::Rbrace) {
                                    opt.push(self.advance().unwrap().lexeme.to_string())
                                }
                                Some(Opt { name, body: opt })
                            } else {
                                None
                            };
                            self.advance();

                            messages.push(MessageDecl {
                                name: msg_name,
                                req,
                                res,
                                body,
                            });
                        } else {
                            self.advance();
                        }
                    }
                    self.advance();

                    ast.synqs.push(PhotonDecl {
                        name,
                        attrs: mem::take(&mut pending_attrs),
                        messages,
                    });
                }
                _ => {
                    self.advance();
                }
            }
        }

        Ok(ast)
    }
}
