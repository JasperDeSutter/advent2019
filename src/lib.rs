#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Opcode {
    Add(bool, bool),
    Mul(bool, bool),
    Exit,
}

fn arg_mode_immediate(instruction: i32, arg: u32) -> bool {
    let decimal_pos = 10i32.pow(arg + 2);
    (instruction / decimal_pos) & 1 == 1
}

impl From<i32> for Opcode {
    fn from(instruction: i32) -> Self {
        let res = instruction % 100;

        match res {
            1 => Opcode::Add(
                arg_mode_immediate(instruction, 0),
                arg_mode_immediate(instruction, 1),
            ),
            2 => Opcode::Mul(
                arg_mode_immediate(instruction, 0),
                arg_mode_immediate(instruction, 1),
            ),
            99 => Opcode::Exit,
            _ => panic!("wrong opcode!"),
        }
    }
}

#[derive(Debug)]
struct ExecuteResult {
    value: i32,
    pos: usize,
    inc: usize,
}

fn get_arg(program: &[i32], value: i32, immediate: bool) -> i32 {
    if immediate {
        value
    } else {
        program[value as usize]
    }
}

impl Opcode {
    fn execute(self, program: &[i32], ptr: usize) -> Option<ExecuteResult> {
        let res = match self {
            Opcode::Add(im0, im1) => ExecuteResult {
                value: get_arg(program, program[ptr], im0)
                    + get_arg(program, program[ptr + 1], im1),
                pos: program[ptr + 2] as usize,
                inc: 3,
            },
            Opcode::Mul(im0, im1) => ExecuteResult {
                value: get_arg(program, program[ptr], im0)
                    * get_arg(program, program[ptr + 1], im1),
                pos: program[ptr + 2] as usize,
                inc: 3,
            },
            _ => None?,
        };
        Some(res)
    }
}

pub fn run_intcode(program: Vec<i32>, instruction_ptr: usize) -> Vec<i32> {
    let opcode = Opcode::from(program[instruction_ptr]);

    if let Some(change) = opcode.execute(&program, instruction_ptr + 1) {
        let next: Vec<_> = program
            .iter()
            .enumerate()
            .map(|(i, val)| if i == change.pos { change.value } else { *val })
            .collect();

        run_intcode(next, instruction_ptr + change.inc + 1)
    } else {
        return program;
    }
}
