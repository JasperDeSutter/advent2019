pub struct Program {
  code: Vec<i64>,
  ptr: usize,
  relative_base: i64,
}

impl Program {
  pub fn new(code: impl Into<Vec<i64>>) -> Self {
    let mut mem: Vec<_> = code.into();
    mem.extend(std::iter::repeat(0).take(1024usize.saturating_sub(mem.len())));
    Self {
      code: mem,
      ptr: 0,
      relative_base: 0,
    }
  }

  fn read(&self, offset: usize) -> i64 {
    self.code[self.ptr + offset]
  }

  fn arg_value(&self, mode: &[ParamMode], idx: usize) -> i64 {
    match mode[idx] {
      ParamMode::Immediate => self.read(idx + 1),
      ParamMode::Position => self.code[self.read(idx + 1) as usize],
      ParamMode::Relative => self.relative_base + self.read(idx + 1),
    }
  }

  pub fn run(&mut self, mut input: impl Iterator<Item = i64>) -> Option<i64> {
    loop {
      match Opcode::from(self.read(0)) {
        Opcode::Add(mode) => {
          let value = self.arg_value(&mode, 0) + self.arg_value(&mode, 1);
          let pos = self.read(3) as usize;
          self.code[pos] = value;
          self.ptr += 3 + 1;
        }
        Opcode::Mul(mode) => {
          let value = self.arg_value(&mode, 0) * self.arg_value(&mode, 1);
          let pos = self.read(3) as usize;
          self.code[pos] = value;
          self.ptr += 3 + 1;
        }
        Opcode::Input => {
          let pos = self.read(1) as usize;
          self.code[pos] = input.next().expect("not enough input");
          self.ptr += 1 + 1;
        }
        Opcode::Output(mode) => {
          let res = self.arg_value(&mode, 0);
          self.ptr += 1 + 1;
          return Some(res);
        }
        Opcode::JumpIfTrue(mode) => {
          self.ptr = match self.arg_value(&mode, 0) {
            0 => self.ptr + 2 + 1,
            _ => self.arg_value(&mode, 1) as usize,
          }
        }
        Opcode::JumpIfFalse(mode) => {
          self.ptr = match self.arg_value(&mode, 0) {
            0 => self.arg_value(&mode, 1) as usize,
            _ => self.ptr + 2 + 1,
          }
        }
        Opcode::LessThan(mode) => {
          let value = self.arg_value(&mode, 0) < self.arg_value(&mode, 1);
          let pos = self.read(3) as usize;
          self.code[pos] = value as i64;
          self.ptr += 3 + 1;
        }
        Opcode::Equals(mode) => {
          let value = self.arg_value(&mode, 0) == self.arg_value(&mode, 1);
          let pos = self.read(3) as usize;
          self.code[pos] = value as i64;
          self.ptr += 3 + 1;
        }
        Opcode::RelativeBase(mode) => {
          let value = self.arg_value(&mode, 0);
          self.relative_base += value;
          self.ptr += 1 + 1;
        }
        Opcode::Exit => return None,
      }
    }
  }

  pub fn code(&self) -> &[i64] {
    &self.code
  }

  pub fn into_code(self) -> Vec<i64> {
    self.code
  }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Opcode {
  Add([ParamMode; 2]),
  Mul([ParamMode; 2]),
  Input,
  Output([ParamMode; 1]),
  JumpIfTrue([ParamMode; 2]),
  JumpIfFalse([ParamMode; 2]),
  LessThan([ParamMode; 2]),
  Equals([ParamMode; 2]),
  RelativeBase([ParamMode; 1]),
  Exit,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum ParamMode {
  Position,
  Immediate,
  Relative,
}

fn arg_mode_immediate(instruction: i64, arg: u32) -> ParamMode {
  let decimal_pos = 10i64.pow(arg + 2);
  match (instruction / decimal_pos) % 10 {
    0 => ParamMode::Position,
    1 => ParamMode::Immediate,
    2 => ParamMode::Relative,
    i => panic!("unknown param mode {} ({})", i, instruction),
  }
}

fn arg_mode_1(instruction: i64) -> [ParamMode; 1] {
  [arg_mode_immediate(instruction, 0)]
}

fn arg_mode_2(instruction: i64) -> [ParamMode; 2] {
  [
    arg_mode_immediate(instruction, 0),
    arg_mode_immediate(instruction, 1),
  ]
}

impl From<i64> for Opcode {
  fn from(instruction: i64) -> Self {
    match instruction % 100 {
      1 => Opcode::Add(arg_mode_2(instruction)),
      2 => Opcode::Mul(arg_mode_2(instruction)),
      3 => Opcode::Input,
      4 => Opcode::Output(arg_mode_1(instruction)),
      5 => Opcode::JumpIfTrue(arg_mode_2(instruction)),
      6 => Opcode::JumpIfFalse(arg_mode_2(instruction)),
      7 => Opcode::LessThan(arg_mode_2(instruction)),
      8 => Opcode::Equals(arg_mode_2(instruction)),
      9 => Opcode::RelativeBase(arg_mode_1(instruction)),
      99 => Opcode::Exit,
      _ => panic!("wrong opcode!"),
    }
  }
}
