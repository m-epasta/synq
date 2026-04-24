use crate::synq::bytecode::{Instruction, Register};

use crate::synq::parser::Ast;

pub fn compile_ast(ast: &Ast) -> Vec<u8> {
    let mut instructions = Vec::new();
    let mut buf = Vec::new();

    for frame in &ast.frames {
        for attr in &frame.attrs {
            let attr_id = map_attribute_key(&attr.key);
            instructions.push(Instruction::LoadAttr {
                dest: Register(0),
                attr_id,
            });
        }

        instructions.push(Instruction::BeginFrame);

        for (i, _field) in frame.fields.iter().enumerate() {
            instructions.push(Instruction::StoreField {
                src: Register(i as u8),
                index: i as u8,
            });
        }
        instructions.push(Instruction::EndFrame { src: Register(0) });
    }

    for synq in &ast.synqs {
        instructions.push(Instruction::BeginPhoton);

        for (i, _msg) in synq.messages.iter().enumerate() {
            instructions.push(Instruction::DefMessage {
                req_frame: i as u8,
                res_frame: (i + 1) as u8,
            });
        }
        instructions.push(Instruction::EndPhoton);
    }

    for inst in instructions {
        inst.emit(&mut buf);
    }

    buf
}

#[inline]
fn map_attribute_key(key: &str) -> u8 {
    match key {
        "codec" => 0x01,
        "over" => 0x02,
        _ => 0xFF, // unknown attribute
    }
}
