use crate::computer::Computer;

pub fn result(computer: &mut Computer) -> Vec<u8> {
    computer.run_program();
    computer.get_output()
}