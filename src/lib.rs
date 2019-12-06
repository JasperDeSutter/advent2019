pub struct Program {
    code: Vec<i32>,
    ptr: usize,
    pub input: Option<i32>,
    pub output: Option<i32>,
}

impl Program {
    pub fn new(code: impl Into<Vec<i32>>) -> Self {
        Self {
            code: code.into(),
            ptr: 0,
            input: None,
            output: None,
        }
    }

    fn read(&self, offset: usize) -> i32 {
        self.code[self.ptr + offset]
    }

    pub fn run(&mut self) {
        while let Some(change) = Opcode::from(self.read(0)).execute(&self) {
            if let Some(pos) = change.pos {
                self.code[pos] = change.value;
            } else {
                self.output = Some(change.value);
            }

            self.ptr += change.inc + 1;
        }
    }

    pub fn code(&self) -> &[i32] {
        &self.code
    }

    pub fn into_code(self) -> Vec<i32> {
        self.code
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Opcode {
    Add(bool, bool),
    Mul(bool, bool),
    Input,
    Output,
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
            3 => Opcode::Input,
            4 => Opcode::Output,
            99 => Opcode::Exit,
            _ => panic!("wrong opcode!"),
        }
    }
}

#[derive(Debug)]
struct ExecuteResult {
    value: i32,
    pos: Option<usize>,
    inc: usize,
}

fn get_arg(program: &Program, value: i32, immediate: bool) -> i32 {
    if immediate {
        value
    } else {
        program.code[value as usize]
    }
}

impl Opcode {
    fn execute(self, program: &Program) -> Option<ExecuteResult> {
        let res = match self {
            Opcode::Add(im0, im1) => ExecuteResult {
                value: get_arg(program, program.read(1), im0)
                    + get_arg(program, program.read(2), im1),
                pos: Some(program.read(3) as usize),
                inc: 3,
            },
            Opcode::Mul(im0, im1) => ExecuteResult {
                value: get_arg(program, program.read(1), im0)
                    * get_arg(program, program.read(2), im1),
                pos: Some(program.read(3) as usize),
                inc: 3,
            },
            Opcode::Input => ExecuteResult {
                value: program.input.unwrap(),
                pos: Some(program.read(1) as usize),
                inc: 1,
            },
            Opcode::Output => ExecuteResult {
                value: get_arg(program, program.read(1), false),
                pos: None,
                inc: 1,
            },
            Opcode::Exit => None?,
        };
        Some(res)
    }
}
