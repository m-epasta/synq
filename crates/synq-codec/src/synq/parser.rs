use crate::{
    error::ParseError,
    synq::lexer::{Token, TokenType},
};
use std::mem;

static FHINTSTABLE: [&str; 4] = ["repeated", "optional", "oneof", "required"];
static TYPETABLE: [&str; 14] = [
    "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "float", "string", "int", "uint",
    "string", "bytes",
];

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

#[derive(Debug)]
pub struct Opt {
    pub name: String,
    pub body: BodyOpt,
}

#[derive(Debug)]
pub struct BodyOpt {
    pub name: String,
    pub namespaces: Option<Vec<String>>,
    pub calls: Vec<Call>,
}

#[derive(Debug)]
pub struct Call {
    pub hint: Option<String>,
    pub fun_name: String,
    pub fun_params: Vec<String>,
    pub is_call: bool,
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
                    let mut pkg = self.advance().unwrap().lexeme.to_string();
                    while self.peek_type() == Some(TokenType::Dot) {
                        self.advance();
                        if let Some(part) = self.advance() {
                            pkg.push('.');
                            pkg.push_str(part.lexeme);
                        }
                    }
                    ast.package.push_str(&pkg);
                }

                TokenType::Import => {
                    self.advance();
                    let mut pkg = self.advance().unwrap().lexeme.to_string();
                    while self.peek_type() == Some(TokenType::Dot) {
                        self.advance();
                        if let Some(part) = self.advance() {
                            pkg.push('.');
                            pkg.push_str(part.lexeme);
                        }
                    }
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
                            let mut reservations = Vec::new();
                            while self.peek_type() != Some(TokenType::Newline)
                                && self.peek_type() != Some(TokenType::Eof)
                            {
                                let nxt = self.advance().unwrap().lexeme;
                                if TYPETABLE.contains(&nxt) {
                                    // em dash is used for convenience
                                    let concate = format!("{}—{nxt}", reservations.last().unwrap());
                                    reservations.push(concate);
                                } else {
                                    reservations.push(self.advance().unwrap().lexeme.to_string());
                                };
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
                            self.advance();
                            let res = self.advance().unwrap().lexeme.to_string();
                            self.advance();

                            self.advance();
                            let body = if self.peek_type() != Some(TokenType::Rbrace) {
                                self.advance();
                                self.advance();
                                let mut opt_name = String::new();
                                while self.peek_type() != Some(TokenType::Lbrace)
                                    && self.peek_type() != Some(TokenType::Eof)
                                {
                                    let curr = self.advance().unwrap().lexeme;
                                    opt_name.push_str(curr);
                                }
                                self.advance();
                                let mut namespaces = Vec::new();
                                while self.peek_type() == Some(TokenType::Use) {
                                    self.advance();
                                    namespaces.push(self.advance().unwrap().lexeme.to_string());
                                }

                                let namespaces = if namespaces.is_empty() {
                                    None
                                } else {
                                    Some(namespaces)
                                };

                                let mut calls = Vec::new();
                                while self.peek_type() != Some(TokenType::Rbrace)
                                    && self.peek_type() != Some(TokenType::Eof)
                                {
                                    if self.peek_type() == Some(TokenType::Newline) {
                                        self.advance();
                                        continue;
                                    }

                                    let tmplexeme = self.peek().unwrap().lexeme;
                                    let hint = if FHINTSTABLE.contains(&tmplexeme) {
                                        self.advance();
                                        Some(tmplexeme.to_string())
                                    } else {
                                        None
                                    };

                                    let mut fun_name = self.advance().unwrap().lexeme.to_string();
                                    while self.peek_type() == Some(TokenType::Dot) {
                                        self.advance();
                                        if let Some(seg) = self.advance() {
                                            fun_name.push('.');
                                            fun_name.push_str(seg.lexeme);
                                        }
                                    }

                                    let mut params = Vec::new();
                                    let is_call;
                                    if self.peek_type() == Some(TokenType::Lparen) {
                                        is_call = true;
                                        self.advance();
                                        while let Some(nxt) = self.peek() {
                                            if nxt.typ == TokenType::Rparen {
                                                self.advance();
                                                break;
                                            }
                                            if nxt.lexeme == "," {
                                                self.advance();
                                                continue;
                                            }
                                            params.push(self.advance().unwrap().lexeme.to_string());
                                        }
                                    } else {
                                        is_call = false;
                                        while let Some(nxt) = self.peek() {
                                            if nxt.typ == TokenType::Newline
                                                || nxt.typ == TokenType::Rbrace
                                                || nxt.typ == TokenType::Eof
                                            {
                                                break;
                                            }
                                            params.push(self.advance().unwrap().lexeme.to_string());
                                        }
                                    }

                                    let call = Call {
                                        hint,
                                        fun_name,
                                        fun_params: params,
                                        is_call,
                                    };

                                    calls.push(call);
                                }

                                let bopt = BodyOpt {
                                    calls,
                                    name: opt_name.clone(),
                                    namespaces,
                                };

                                Some(Opt {
                                    name: name.clone(),
                                    body: bopt,
                                })
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
