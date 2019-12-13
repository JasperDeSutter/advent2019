pub struct Program {
  code: Vec<i64>,
  ptr: usize,
  relative_base: i64,
}

impl Program {
  pub fn new(code: impl Into<Vec<i64>>) -> Self {
    const MIN_MEMORY: usize = 4096;

    let mut mem: Vec<_> = code.into();
    mem.extend(std::iter::repeat(0).take(MIN_MEMORY.saturating_sub(mem.len())));
    Self {
      code: mem,
      ptr: 0,
      relative_base: 0,
    }
  }

  fn offset_value(&self, mode: ParamMode, offset_value: i64) -> Option<usize> {
    match mode {
      ParamMode::Position => Some(offset_value as usize),
      ParamMode::Relative => Some((self.relative_base + offset_value) as usize),
      _ => None,
    }
  }

  fn arg_value(&self, mode: &[ParamMode], idx: usize) -> i64 {
    let offset_value = self.code[self.ptr + idx + 1];
    match self.offset_value(mode[idx], offset_value) {
      Some(offset) => self.code[offset],
      None => offset_value,
    }
  }

  fn write(&mut self, mode: &[ParamMode], value: i64) {
    let idx = mode.len() - 1;
    let offset_value = self.code[self.ptr + idx + 1];
    let offset = self.offset_value(mode[idx], offset_value).unwrap();
    self.code[offset] = value;
    self.ptr += mode.len() + 1;
  }

  #[inline]
  fn jump(&mut self, jump: bool, mode: &[ParamMode]) {
    self.ptr = if jump {
      self.arg_value(&mode, mode.len() - 1) as usize
    } else {
      self.ptr + mode.len() + 1
    };
  }

  pub fn run(&mut self, mut input: impl Iterator<Item = i64>) -> Option<i64> {
    loop {
      match Opcode::from(self.code[self.ptr]) {
        Opcode::Add(mode) => {
          let value = self.arg_value(&mode, 0) + self.arg_value(&mode, 1);
          self.write(&mode, value);
        }
        Opcode::Mul(mode) => {
          let value = self.arg_value(&mode, 0) * self.arg_value(&mode, 1);
          self.write(&mode, value);
        }
        Opcode::LessThan(mode) => {
          let value = self.arg_value(&mode, 0) < self.arg_value(&mode, 1);
          self.write(&mode, value as i64);
        }
        Opcode::Equals(mode) => {
          let value = self.arg_value(&mode, 0) == self.arg_value(&mode, 1);
          self.write(&mode, value as i64);
        }
        Opcode::Input(mode) => {
          let value = input.next().expect("not enough input");
          self.write(&mode, value);
        }
        Opcode::Output(mode) => {
          let res = self.arg_value(&mode, 0);
          self.ptr += mode.len() + 1;
          return Some(res);
        }
        Opcode::JumpIfTrue(mode) => self.jump(self.arg_value(&mode, 0) != 0, &mode),
        Opcode::JumpIfFalse(mode) => self.jump(self.arg_value(&mode, 0) == 0, &mode),
        Opcode::RelativeBase(mode) => {
          self.relative_base += self.arg_value(&mode, 0);
          self.ptr += mode.len() + 1;
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
  Add([ParamMode; 3]),
  Mul([ParamMode; 3]),
  Input([ParamMode; 1]),
  Output([ParamMode; 1]),
  JumpIfTrue([ParamMode; 2]),
  JumpIfFalse([ParamMode; 2]),
  LessThan([ParamMode; 3]),
  Equals([ParamMode; 3]),
  RelativeBase([ParamMode; 1]),
  Exit,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum ParamMode {
  Position,
  Immediate,
  Relative,
}

#[inline]
fn arg_mode_immediate(instruction: i64, arg: u32) -> ParamMode {
  let decimal_pos = 10i64.pow(arg + 2);
  match (instruction / decimal_pos) % 10 {
    0 => ParamMode::Position,
    1 => ParamMode::Immediate,
    2 => ParamMode::Relative,
    i => panic!("unknown param mode {} ({})", i, instruction),
  }
}

macro_rules! arg_mode {
  ($i:expr => $($x:expr),*) => ([
    $(arg_mode_immediate($i, $x)),*
  ]);
}

impl From<i64> for Opcode {
  fn from(instruction: i64) -> Self {
    fn arg1(i: i64) -> [ParamMode; 1] {
      arg_mode![i => 0]
    }
    fn arg2(i: i64) -> [ParamMode; 2] {
      arg_mode![i => 0, 1]
    }
    fn arg3(i: i64) -> [ParamMode; 3] {
      arg_mode![i => 0, 1, 2]
    }

    match instruction % 100 {
      1 => Opcode::Add(arg3(instruction)),
      2 => Opcode::Mul(arg3(instruction)),
      3 => Opcode::Input(arg1(instruction)),
      4 => Opcode::Output(arg1(instruction)),
      5 => Opcode::JumpIfTrue(arg2(instruction)),
      6 => Opcode::JumpIfFalse(arg2(instruction)),
      7 => Opcode::LessThan(arg3(instruction)),
      8 => Opcode::Equals(arg3(instruction)),
      9 => Opcode::RelativeBase(arg1(instruction)),
      99 => Opcode::Exit,
      i => panic!("wrong opcode {}! ({})", i, instruction),
    }
  }
}
