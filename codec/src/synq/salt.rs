// use crate::{
//     error::ParseError,
//     synq::parser::{Ast, FieldDecl, FrameDecl, PhotonDecl},
// };
//
// pub(crate) struct Salter;
//
// impl Salter {
//     pub(crate) fn new() -> Self {
//         Self
//     }
//
//     // Salting does not affect package, import and const
//     pub(crate) fn salt(&self, ast: Ast) -> Result<Ast, ParseError> {
//         let frames = self.frames(&ast.frames);
//         // let synqs = self.synqs(&ast.synqs);
//
//         Ok(ast)
//     }
//
//     // fn frames(&self, frames: &Vec<FrameDecl>) -> Vec<FrameDecl> {
//     //     for frame in frames {
//     //         self.visit_fields(frame.fields);
//     //     }
//     // }
//
//     fn visit_fields(&self, fields: Vec<FieldDecl>) -> Vec<FieldDecl> {
//         for field in fields {}
//     }
//
//     // fn synqs(&self, synqs: &Vec<PhotonDecl>) -> Vec<PhotonDecl> {}
// }
