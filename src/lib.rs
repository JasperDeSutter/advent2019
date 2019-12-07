pub struct Program<'a> {
  code: Vec<i32>,
  ptr: usize,
  pub input: Box<dyn Iterator<Item = i32> + 'a>,
  pub output: Option<i32>,
}

impl<'a> Program<'a> {
  pub fn new(code: impl Into<Vec<i32>>) -> Self {
    Self {
      code: code.into(),
      ptr: 0,
      input: Box::new(std::iter::empty()),
      output: None,
    }
  }

  fn read(&self, offset: usize) -> i32 {
    self.code[self.ptr + offset]
  }

  fn arg_value(&self, mode: &[ParamMode], idx: usize) -> i32 {
    match mode[idx] {
      ParamMode::Immediate => self.read(idx + 1),
      ParamMode::Position => self.code[self.read(idx + 1) as usize],
    }
  }

  pub fn run(&mut self) {
    assert!(self.ptr == 0, "can't run programs twice!");
    loop {
      let opcode = Opcode::from(self.read(0));
      match opcode {
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
          self.code[pos] = self.input.next().expect("not enough input");
          self.ptr += 1 + 1;
        }
        Opcode::Output(mode) => {
          self.output = Some(self.arg_value(&mode, 0));
          self.ptr += 1 + 1;
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
          self.code[pos] = value as i32;
          self.ptr += 3 + 1;
        }
        Opcode::Equals(mode) => {
          let value = self.arg_value(&mode, 0) == self.arg_value(&mode, 1);
          let pos = self.read(3) as usize;
          self.code[pos] = value as i32;
          self.ptr += 3 + 1;
        }
        Opcode::Exit => break,
      }
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
  Add([ParamMode; 2]),
  Mul([ParamMode; 2]),
  Input,
  Output([ParamMode; 1]),
  JumpIfTrue([ParamMode; 2]),
  JumpIfFalse([ParamMode; 2]),
  LessThan([ParamMode; 2]),
  Equals([ParamMode; 2]),
  Exit,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum ParamMode {
  Position,
  Immediate,
}

fn arg_mode_immediate(instruction: i32, arg: u32) -> ParamMode {
  let decimal_pos = 10i32.pow(arg + 2);
  match (instruction / decimal_pos) & 1 {
    0 => ParamMode::Position,
    _ => ParamMode::Immediate,
  }
}

fn arg_mode_1(instruction: i32) -> [ParamMode; 1] {
  [arg_mode_immediate(instruction, 0)]
}

fn arg_mode_2(instruction: i32) -> [ParamMode; 2] {
  [
    arg_mode_immediate(instruction, 0),
    arg_mode_immediate(instruction, 1),
  ]
}

impl From<i32> for Opcode {
  fn from(instruction: i32) -> Self {
    match instruction % 100 {
      1 => Opcode::Add(arg_mode_2(instruction)),
      2 => Opcode::Mul(arg_mode_2(instruction)),
      3 => Opcode::Input,
      4 => Opcode::Output(arg_mode_1(instruction)),
      5 => Opcode::JumpIfTrue(arg_mode_2(instruction)),
      6 => Opcode::JumpIfFalse(arg_mode_2(instruction)),
      7 => Opcode::LessThan(arg_mode_2(instruction)),
      8 => Opcode::Equals(arg_mode_2(instruction)),
      99 => Opcode::Exit,
      _ => panic!("wrong opcode!"),
    }
  }
}
