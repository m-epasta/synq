use crate::synq::parser::Ast;

pub struct AstPrinter;

impl AstPrinter {
    pub fn debug(ast: &Ast) {
        println!("{ast:?}")
    }

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
                    if let Some(opt) = &msg.body {
                        println!("      option ({}) {{", opt.body.name);
                        if let Some(namespaces) = &opt.body.namespaces {
                            for ns in namespaces {
                                println!("        use {}", ns);
                            }
                        }
                        for call in &opt.body.calls {
                            let hint = call
                                .hint
                                .as_ref()
                                .map(|h| format!("{} ", h))
                                .unwrap_or_default();
                            if call.is_call {
                                println!(
                                    "        {}{}({})",
                                    hint,
                                    call.fun_name,
                                    call.fun_params.join(", ")
                                );
                            } else {
                                println!(
                                    "        {}{} {}",
                                    hint,
                                    call.fun_name,
                                    call.fun_params.join(" ")
                                );
                            }
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

    // #[test]
    // fn test_print_frame() {
    //     let input = "@over NATS\nframe Block {\n  sender u64\n  receiver u64\n  amount f64\n}";
    //     let ast = parse_synq(input);
    //     println!("\n\n\n");
    //     AstPrinter::print(&ast);
    //     let mermaid = AstPrinter::to_mermaid(&ast);
    //     assert!(mermaid.contains("Frame: Block"));
    //     assert!(mermaid.contains("sender: u64"));
    //     assert!(mermaid.contains("receiver: u64"));
    //     assert!(mermaid.contains("amount: f64"));
    // }
    //
    // #[test]
    // fn test_print_synq() {
    //     println!("\n\n\n");
    //     let input =
    //         "synq Block {\n  message get_id (Addr) (Id) {}\n}\nframe Addr {\n  name string\n}";
    //     let ast = parse_synq(input);
    //     AstPrinter::print(&ast);
    //     let mermaid = AstPrinter::to_mermaid(&ast);
    //     assert!(mermaid.contains("Photon: Block"));
    //     assert!(mermaid.contains("Message: get_id"));
    //     assert!(mermaid.contains("Req: Addr"));
    //     assert!(mermaid.contains("Res: Id"));
    //     assert!(mermaid.contains("Frame: Addr"));
    // }

    #[test]
    fn test_option_and_photon() {
        let input =
        "package block.demo\nimport chain\nsynq Block {  \nmessage check_transaction(TrMetadata) returns (TrAcceptance) {    \noption (chain.check) {        \n// param gets automatically namespaced,        \n// To avoid conflicts (undefined behaviour) you may namespace them yourself if multiple arguments        \n// Like: use TrMetadata or px where x is the param index (starting at 1)        \nrequire chain.check.valid_addr(sender)        \nrequire chain.check.valid_addr(receiver)        \nrequire chain.check.no_trunc(amount)        \nrequire trusted >= 128    \n}  \n}\n}\n\nframe TrMetadata {  \nsender u64  \nreceiver u64  \nrequired amount u64  \ntrusted i8\n}\n\nframe TrAcceptance {    \nsender bool    \nreceiver bool    \namount bool    \ntrusted i8\n}
        ";
        let ast = parse_synq(input);
        AstPrinter::debug(&ast);
        AstPrinter::print(&ast);
        // AstPrinter::to_mermaid(&ast);
    }

    // #[test]
    // fn test_reservations() {
    //     let input = "frame Sender {\n   wallet u64\n    required amount u64\n}\n";
    //     let ast = parse_synq(input);
    //     AstPrinter::debug(&ast);
    // }
}
