use rspirv_reflect::{
    Reflection,
    rspirv::dr::{Instruction, Operand},
    spirv::Op,
};

use super::Type;

/// A parsed `OpTypeArray`.
#[derive(Debug)]
pub struct Array {
    pub element_type: Box<Type>, // Any non-void type
    pub length: u32,
}

impl Array {
    pub fn parse_instruction(instruction: &Instruction, spirv: &Reflection) -> Option<Self> {
        if !matches!(instruction.class.opcode, Op::TypeArray) {
            return None;
        }

        let Some(Operand::IdRef(element_type_id)) = instruction.operands.first() else {
            return None;
        };

        let element_type = spirv.0.types_global_values.iter().find_map(|instruction| {
            if instruction.result_id.unwrap_or(u32::MAX) == *element_type_id {
                Type::parse_instruction(instruction, spirv)
            } else {
                None
            }
        })?;

        let Some(Operand::LiteralBit32(length)) = instruction.operands.get(1) else {
            return None;
        };

        Some(Self {
            element_type: Box::new(element_type),
            length: *length,
        })
    }

    pub fn size(&self) -> usize {
        let element_size = self.element_type.size();

        // TODO elements may have padding between them
        // ArrayStride decorations

        element_size * self.length as usize
    }

    pub fn alignment(&self) -> usize {
        self.element_type.alignment()
    }

    pub fn type_syntax(&self) -> syn::Type {
        let element_type = self.element_type.type_syntax();
        let length = self.length as usize;

        // TODO how to handle element padding? Tuples?

        syn::parse_quote! {[#element_type; #length]}
    }
}
