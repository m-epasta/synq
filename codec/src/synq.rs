use crate::synq::lexer::{Token, TokenType};
use crate::synq::parser::Parser;
use crate::{error::ParseError, synq::scanner::Scanner};

pub mod bytecode;
pub mod compiler;
pub mod lexer;
pub mod parser;
mod salt;
pub mod scanner;
pub mod ast_printer;

pub fn compile(input: String) -> Result<Vec<u8>, ParseError> {
    let mut scanner = Scanner::new(&input);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token()?;
        let is_eof = token.typ == TokenType::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    if std::env::var("PHOTON_PRINT_TOKSTREAM").is_ok() {
        print_tokens(&tokens);
    }

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse()?;

    if std::env::var("PHOTON_PRINT_AST").is_ok() {
        ast_printer::AstPrinter::print(&ast);
        println!("\nMermaid Diagram:\n{}", ast_printer::AstPrinter::to_mermaid(&ast));
    }

    Ok(compiler::compile_ast(&ast))
}

fn print_tokens(tokens: &Vec<Token>) {
    let mut indent = 0usize;

    for token in tokens {
        if token.typ == TokenType::Rbrace {
            indent = indent.saturating_sub(2);
        }

        println!(
            "{}Token({:?}, {:?})",
            " ".repeat(indent),
            token.typ,
            token.lexeme
        );

        if token.typ == TokenType::Lbrace {
            indent += 2;
        }

        if token.typ == TokenType::Eof {
            break;
        }
    }
}
