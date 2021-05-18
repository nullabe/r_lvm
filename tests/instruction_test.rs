#[cfg(test)]
mod instruction_test {
    use r_lvm::instruction::*;

    #[test]
    fn new_hlt_opcode() {
        let hlt = Opcode::Hlt;

        assert_eq!(hlt, Opcode::Hlt);
    }

    #[test]
    fn new_instruction_with_only_an_htl_opcode() {
        let instruction = Instruction::new(Opcode::Hlt);

        assert_eq!(instruction.get_opcode_as_ref(), &Opcode::Hlt);
    }
}
