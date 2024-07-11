#[derive(Debug, PartialEq)]
enum Instruction {
    Move(Register, Register),
}

#[derive(Debug, PartialEq, Clone)]
enum Register {
    AL, BL, CL, DL, AH, BH, CH, DH,
    AX, BX, CX, DX, SP, BP, SI, DI
}

const REGISTERS: [[Register; 2]; 8] = [
    [Register::AL, Register::AX], // 000
    [Register::CL, Register::CX], // 001
    [Register::DL, Register::DX], // 010
    [Register::BL, Register::BX], // 011
    [Register::AH, Register::SP], // 100
    [Register::CH, Register::BP], // 101
    [Register::DH, Register::SI], // 110
    [Register::BH, Register::DI], // 111
];

fn decode(instr_set: &[u8;2]) -> Instruction {
    let higher_bits = instr_set[0];
    let lower_bits = instr_set[1];

    let word = (higher_bits & 0b00000001) as usize;
    let destination = lower_bits & 0b00000111;
    let source = (lower_bits & 0b00111000) >> 3;

    let destination_register = REGISTERS.get(destination as usize).unwrap().get(word).unwrap();
    let source_register =  REGISTERS.get(source as usize).unwrap().get(word).unwrap();

    Instruction::Move(destination_register.clone(), source_register.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_register_move() {
        let single_register_move:&[u8;2] = include_bytes!("../fixture/listing_0037_single_register_mov");

        let instr = decode(single_register_move);

        assert_eq!(instr, Instruction::Move(Register::CX, Register::BX));
    }

    #[test]
    fn test_many_register_move() {
        let many_register_move:&[u8;22] = include_bytes!("../fixture/listing_0038_many_register_mov");

        let expected = vec![
            Instruction::Move(Register::CX, Register::BX),
            Instruction::Move(Register::CH, Register::AH),
            Instruction::Move(Register::DX, Register::BX),
            Instruction::Move(Register::SI, Register::BX),
            Instruction::Move(Register::BX, Register::DI),
            Instruction::Move(Register::AL, Register::CL),
            Instruction::Move(Register::CH, Register::CH),
            Instruction::Move(Register::BX, Register::AX),
            Instruction::Move(Register::BX, Register::SI),
            Instruction::Move(Register::SP, Register::DI),
            Instruction::Move(Register::BP, Register::AX),
        ];

        for count in 0..expected.len() {
            let instr = decode(&[many_register_move[count * 2], many_register_move[(count * 2) + 1]]);
            assert_eq!(instr, expected[count]);
        }
    }
}
