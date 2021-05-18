#[cfg(test)]
mod virtual_machine_test {
    use r_lvm::virtual_machine::*;

    #[test]
    pub fn new_virtual_machine() {
        let vm = VirtualMachine::new();

        assert_eq!(vm.get_register()[0], 0);
        assert_eq!(vm.get_register().len(), 32);
    }

    #[test]
    pub fn default_virtual_machine() {
        let vm = VirtualMachine::default();

        assert_eq!(vm.get_register()[0], 0);
        assert_eq!(vm.get_register().len(), 32);
    }

    #[test]
    pub fn run_virtual_machine_hlt() {
        let mut vm = VirtualMachine::new();

        vm.set_bytes(vec![0, 0, 0, 0]);

        vm.run();

        assert_eq!(vm.get_byte_index(), 1);
    }

    #[test]
    pub fn run_virtual_machine_igl() {
        let mut vm = VirtualMachine::new();

        vm.set_bytes(vec![200, 0, 0, 0]);

        vm.run();

        assert_eq!(vm.get_byte_index(), 1);
    }
}
