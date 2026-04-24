#[cfg(test)]
use codec::synq::{lexer::TokenType, scanner::Scanner};

#[test]
fn lexing_frame_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = std::path::PathBuf::from(manifest_dir).join("files/frame.synq");
    let content = std::fs::read_to_string(path)?;
    let mut scanner = Scanner::new(&content);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token()?;
        let is_eof = token.typ == TokenType::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    for tok in &tokens {
        print!("  {tok:?}  ");
    }

    Ok(())
}

#[test]
fn lexing_synq_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = std::path::PathBuf::from(manifest_dir).join("files/synq.synq");
    let content = std::fs::read_to_string(path)?;
    let mut scanner = Scanner::new(&content);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token()?;
        let is_eof = token.typ == TokenType::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    for tok in tokens {
        print!("  {tok:?}  ");
    }

    Ok(())
}

#[test]
fn lexing_option_succeed() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = std::path::PathBuf::from(manifest_dir).join("files/option.synq");
    let content = std::fs::read_to_string(path)?;
    let mut scanner = Scanner::new(&content);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token()?;
        let is_eof = token.typ == TokenType::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    for tok in tokens {
        print!("  {tok:?}  ");
    }

    Ok(())
}

#[test]
fn lexing_message_succeed() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let path = std::path::PathBuf::from(manifest_dir).join("files/message.synq");
    let content = std::fs::read_to_string(path)?;
    let mut scanner = Scanner::new(&content);
    let mut tokens = Vec::new();

    loop {
        let token = scanner.next_token()?;
        let is_eof = token.typ == TokenType::Eof;
        tokens.push(token);
        if is_eof {
            break;
        }
    }

    for tok in tokens {
        print!("  {tok:?}  ");
    }

    Ok(())
}
