use std::{fs, env};

fn main() {

    let path = env::args().nth(1).expect("Input file name");
    let expected_result = env::args().nth(2).map(|b| b.parse::<u32>().unwrap());
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let ram: Result<Vec<u32>, _> = contents.split(',')
                    .map(|code| code.parse())
                    .collect();
    let ram = ram.unwrap();

    if let Some(ex) = expected_result {
        for state in 0..=9999 {
            match execute_program(&ram, state) {
                Ok(res) if res == ex => {
                            println!("Valid initial state is = {}", state);
                            return
                        }
                r => {
                    println!("Excluded state {}: {:?}", state, r);       
                }
            }
        }
    } else {
        let result = execute_program(&ram, 1202).expect("Should find hatl!");
        println!("Value at positioon 0 = {}", result)
    }
}

fn execute_program(program: &[u32], initial_state: u32) -> Result<u32, String> {
    let mut ram = program.to_vec();

    ram[1] = initial_state/100;
    ram[2] = initial_state%100;

    run_program(ram.as_mut_slice(), 0)?;

    Ok(ram[0])
}

#[derive(Debug, PartialEq, Eq)]
enum Asm {
    Nop,
    Halt,
    Add(usize, usize, usize),
    Mult(usize, usize, usize),
    Error(u32),
}

impl Asm {
    fn len(&self) -> usize {
        use Asm::*;
        match self {
            Nop | Halt | Error(_) => 1,
            Add(_,_,_) | Mult(_,_,_) => 4
        }
    }
}

fn get_op_code(code: &[u32]) -> Asm {
    use Asm::*;
    match code[0] {
        0 => Nop,
        1 => Add(code[1] as usize, code[2] as usize, code[3] as usize),
        2 => Mult(code[1] as usize, code[2] as usize, code[3] as usize),
        99 => Halt,
        unknow => Error(unknow)
    }
}

fn step(ram: &mut [u32], pos: usize) -> Asm {
    use Asm::*;
    let opcode = get_op_code(&ram[pos..]);
    match opcode {
        Add(a, b, dest) => ram[dest] = ram[a] + ram[b],
        Mult(a, b, dest) => ram[dest] = ram[a] * ram[b],
        _ => {}
    }
    opcode
}

pub fn run_program(ram: &mut [u32], mut pc: usize) -> Result<(), String> {
    loop {
        match step(ram, pc) {
            Asm::Halt => return Ok(()),
            Asm::Error(opcode) => return Err(format!("Unknow code '{}'", opcode)),
            opcode => pc += opcode.len()
        }
    }
}


#[cfg(test)]
mod intcode_shoud {
    use super::*;
    use rstest::*;

    #[rstest(
        code, asm,
        case(vec![99], Asm::Halt),
        case(vec![1,2,3,4], Asm::Add(2, 3, 4)),
        case(vec![2,4,3,2], Asm::Mult(4, 3, 2)),
        case(vec![0], Asm::Nop),
        case(vec![43], Asm::Error(43)),
    )]
    fn parse(code: impl AsRef<[u32]>, asm: Asm) {
        assert_eq!(asm, get_op_code(code.as_ref()));
    }

    #[rstest(
        asm, len, 
        case(Asm::Halt, 1),
        case(Asm::Add(2, 3, 4), 4),
        case(Asm::Mult(4, 3, 2), 4),
        case(Asm::Nop, 1),
        case(Asm::Error(43), 1),
    )]
    fn asm_len(asm: Asm, len: usize) {
        assert_eq!(asm.len(), len);
    }

    #[rstest(
        ram, pos, expected, executed,
        case::add(vec![1,9,10,3,2,3,11,0,99,30,40,50], 0, vec![1,9,10,70,2,3,11,0,99,30,40,50], Asm::Add(9, 10, 3)),
        case::mult(vec![1,9,10,70,2,3,11,0,99,30,40,50], 4, vec![3500,9,10,70,2,3,11,0,99,30,40,50], Asm::Mult(3, 11, 0)),
        case::halt(vec![3500,9,10,70,2,3,11,0,99,30,40,50], 8, vec![3500,9,10,70,2,3,11,0,99,30,40,50], Asm::Halt),
        case::nop(vec![3500,9,10,70,2,3,11,0,99,30,40,50], 7, vec![3500,9,10,70,2,3,11,0,99,30,40,50], Asm::Nop),
        case::error(vec![3500,9,10,70,2,3,11,0,99,30,40,50], 1, vec![3500,9,10,70,2,3,11,0,99,30,40,50], Asm::Error(9)),
    )]
    fn single_step(mut ram: Vec<u32>, pos: usize, expected: Vec<u32>, executed: Asm) {
        let opcode = step(ram.as_mut_slice(), pos);

        assert_eq!(ram, expected);
        assert_eq!(opcode, executed);
    }

    #[test]
    fn run() {
        let mut ram = vec![1,9,10,3,2,3,11,0,99,30,40,50];

        assert!(run_program(ram.as_mut_slice(), 0).is_ok());

        assert_eq!(ram, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
    }
}