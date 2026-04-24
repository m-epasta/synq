#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // I/O Instructions
    ReadU8 = 0x01,
    WriteU8 = 0x02,
    ReadU64 = 0x03,
    WriteU64 = 0x04,
    ReadF64 = 0x05,
    WriteF64 = 0x06,
    ReadVarint = 0x07,
    WriteVarint = 0x08,
    ReadString = 0x09,
    WriteString = 0x0A,

    BeginFrame = 0x10,
    EndFrame = 0x11,
    StoreField = 0x12,
    LoadField = 0x13,

    BeginPhoton = 0x14,
    EndPhoton = 0x15,
    DefMessage = 0x16,

    // Modifiers and Control Flow
    LoadAttr = 0x20,
    JumpIfNull = 0x21,
    Yield = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // Memory and Stream interaction
    ReadU8 { dest: Register },
    WriteU8 { src: Register },
    ReadU64 { dest: Register },
    WriteU64 { src: Register },
    ReadF64 { dest: Register },
    WriteF64 { src: Register },
    ReadVarint { dest: Register },
    WriteVarint { src: Register },
    ReadString { dest: Register },
    WriteString { src: Register },

    // Struct / Schema management
    BeginFrame,
    EndFrame { src: Register },
    StoreField { src: Register, index: u8 },
    LoadField { dest: Register, index: u8 },

    // Service & RPC management
    BeginPhoton,
    EndPhoton,
    DefMessage { req_frame: u8, res_frame: u8 },

    // Configuration
    LoadAttr { dest: Register, attr_id: u8 },
    Yield { src: Register },
}

impl Instruction {
    pub fn emit(&self, buf: &mut Vec<u8>) {
        match self {
            Self::ReadU64 { dest } => {
                buf.push(OpCode::ReadU64 as u8);
                buf.push(dest.0);
            }
            Self::WriteU64 { src } => {
                buf.push(OpCode::WriteU64 as u8);
                buf.push(src.0);
            }
            Self::ReadF64 { dest } => {
                buf.push(OpCode::ReadF64 as u8);
                buf.push(dest.0);
            }
            Self::WriteF64 { src } => {
                buf.push(OpCode::WriteF64 as u8);
                buf.push(src.0);
            }
            Self::BeginFrame => {
                buf.push(OpCode::BeginFrame as u8);
            }
            Self::EndFrame { src } => {
                buf.push(OpCode::EndFrame as u8);
                buf.push(src.0);
            }
            Self::StoreField { src, index } => {
                buf.push(OpCode::StoreField as u8);
                buf.push(src.0);
                buf.push(*index);
            }
            Self::LoadField { dest, index } => {
                buf.push(OpCode::LoadField as u8);
                buf.push(dest.0);
                buf.push(*index);
            }
            Self::BeginPhoton => {
                buf.push(OpCode::BeginPhoton as u8);
            }
            Self::EndPhoton => {
                buf.push(OpCode::EndPhoton as u8);
            }
            Self::DefMessage { req_frame, res_frame } => {
                buf.push(OpCode::DefMessage as u8);
                buf.push(*req_frame);
                buf.push(*res_frame);
            }
            Self::LoadAttr { dest, attr_id } => {
                buf.push(OpCode::LoadAttr as u8);
                buf.push(dest.0);
                buf.push(*attr_id);
            }
            Self::Yield { src } => {
                buf.push(OpCode::Yield as u8);
                buf.push(src.0);
            }
            _ => {
                todo!("Emit for {:?}", self)
            }
        }
    }
}
