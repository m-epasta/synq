use crate::synq::parser::Ast;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(ast: &Ast) {
        println!("AST for package: {}", ast.package);

        if !ast.import.is_empty() {
            println!("\nImports:");
            for import in &ast.import {
                println!("  - {}", import);
            }
        }

        if !ast.options.is_empty() {
            println!("\nOptions:");
            for opt in &ast.options {
                println!("  {} = {}", opt.name, opt.hint);
            }
        }

        if !ast.consts.is_empty() {
            println!("\nConstants:");
            for c in &ast.consts {
                println!("  const {} = {}", c.name, c.value);
            }
        }

        if !ast.enums.is_empty() {
            println!("\nEnums:");
            for e in &ast.enums {
                println!("  enum {} {{", e.name);
                for c in &e.consts {
                    println!("    {} = {}", c.name, c.value);
                }
                println!("  }}");
            }
        }

        if !ast.frames.is_empty() {
            println!("\nFrames:");
            for f in &ast.frames {
                println!("  frame {} {{", f.name);
                for attr in &f.attrs {
                    println!("    @{}({})", attr.key, attr.val);
                }
                for field in &f.fields {
                    let hint = field
                        .hint
                        .as_ref()
                        .map(|h| format!("{} ", h))
                        .unwrap_or_default();
                    println!("    {}{} {}", hint, field.name, field.typ);
                }
                for res in &f.reservations {
                    println!("    reserved {}", res.reservations.join(", "));
                }
                println!("  }}");
            }
        }

        if !ast.synqs.is_empty() {
            println!("\nPhotons:");
            for p in &ast.synqs {
                println!("  synq {} {{", p.name);
                for attr in &p.attrs {
                    println!("    @{}({})", attr.key, attr.val);
                }
                for msg in &p.messages {
                    println!("    message {} {{", msg.name);
                    println!("      req: {}", msg.req);
                    println!("      res: {}", msg.res);
                    if let Some(body) = &msg.body {
                        println!("      body {} {{", body.name);
                        for line in &body.body {
                            println!("        {}", line);
                        }
                        println!("      }}");
                    }
                    println!("    }}");
                }
                println!("  }}");
            }
        }
    }

    pub fn to_mermaid(ast: &Ast) -> String {
        let mut mm = String::from("graph TD\n");
        mm.push_str(&format!("  Root[Package: {}]\n", ast.package));

        if !ast.consts.is_empty() {
            mm.push_str("  Root --> Consts[Constants]\n");
            for c in &ast.consts {
                mm.push_str(&format!(
                    "  Consts --> C_{}[{}: {}]\n",
                    c.name, c.name, c.value
                ));
            }
        }

        if !ast.enums.is_empty() {
            mm.push_str("  Root --> Enums[Enums]\n");
            for e in &ast.enums {
                mm.push_str(&format!("  Enums --> E_{}[Enum: {}]\n", e.name, e.name));
                for c in &e.consts {
                    mm.push_str(&format!(
                        "  E_{} --> EC_{}_{}[{}: {}]\n",
                        e.name, e.name, c.name, c.name, c.value
                    ));
                }
            }
        }

        if !ast.frames.is_empty() {
            mm.push_str("  Root --> Frames[Frames]\n");
            for f in &ast.frames {
                mm.push_str(&format!("  Frames --> F_{}[Frame: {}]\n", f.name, f.name));
                for field in &f.fields {
                    let label = format!(
                        "{}{}: {}",
                        field
                            .hint
                            .as_ref()
                            .map(|h| format!("{} ", h))
                            .unwrap_or_default(),
                        field.name,
                        field.typ
                    );
                    mm.push_str(&format!(
                        "  F_{} --> FF_{}_{}[\"{}\"]\n",
                        f.name, f.name, field.name, label
                    ));
                }
            }
        }

        if !ast.synqs.is_empty() {
            mm.push_str("  Root --> Photons[Photons]\n");
            for p in &ast.synqs {
                mm.push_str(&format!("  Photons --> P_{}[Photon: {}]\n", p.name, p.name));
                for msg in &p.messages {
                    mm.push_str(&format!(
                        "  P_{} --> PM_{}_{}[Message: {}]\n",
                        p.name, p.name, msg.name, msg.name
                    ));
                    mm.push_str(&format!(
                        "  PM_{}_{} --> PMR_{}_{}_Q[Req: {}]\n",
                        p.name, msg.name, p.name, msg.name, msg.req
                    ));
                    mm.push_str(&format!(
                        "  PM_{}_{} --> PMR_{}_{}_S[Res: {}]\n",
                        p.name, msg.name, p.name, msg.name, msg.res
                    ));
                }
            }
        }

        mm
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synq::lexer::TokenType;
    use crate::synq::parser::Parser;
    use crate::synq::scanner::Scanner;

    fn parse_synq(input: &str) -> Ast {
        let mut scanner = Scanner::new(input);
        let mut tokens = Vec::new();

        loop {
            let token = scanner.next_token().unwrap();
            let is_eof = token.typ == TokenType::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        let mut parser = Parser::new(&tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_print_frame() {
        let input = "@over NATS\nframe Block {\n  sender u64\n  receiver u64\n  amount f64\n}";
        let ast = parse_synq(input);
        AstPrinter::print(&ast);
        let mermaid = AstPrinter::to_mermaid(&ast);
        assert!(mermaid.contains("Frame: Block"));
        assert!(mermaid.contains("sender: u64"));
        assert!(mermaid.contains("receiver: u64"));
        assert!(mermaid.contains("amount: f64"));
    }

    #[test]
    fn test_print_synq() {
        let input =
            "synq Block {\n  message get_id (Addr) (Id) {}\n}\nframe Addr {\n  name string\n}";
        let ast = parse_synq(input);
        AstPrinter::print(&ast);
        let mermaid = AstPrinter::to_mermaid(&ast);
        assert!(mermaid.contains("Photon: Block"));
        assert!(mermaid.contains("Message: get_id"));
        assert!(mermaid.contains("Req: Addr"));
        assert!(mermaid.contains("Res: Id"));
        assert!(mermaid.contains("Frame: Addr"));
    }
}
