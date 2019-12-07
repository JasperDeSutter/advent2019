use advent19::Program;

#[rustfmt::skip]
const GRAVITY_ASSIST_PROGRAM: &[i32] = &[
  1, 0, 0, 3,
  1, 1, 2, 3,
  1, 3, 4, 3,
  1, 5, 0, 3,
  2, 1, 10, 19,
  1, 6, 19, 23,
  1, 10, 23, 27,
  2, 27, 13, 31,
  1, 31, 6, 35,
  2, 6, 35, 39,
  1, 39, 5, 43,
  1, 6, 43, 47,
  2, 6, 47, 51,
  1, 51, 5, 55,
  2, 55, 9, 59,
  1, 6, 59, 63,
  1, 9, 63, 67,
  1, 67, 10, 71,
  2, 9, 71, 75,
  1, 6, 75, 79,
  1, 5, 79, 83,
  2, 83, 10, 87,
  1, 87, 5, 91,
  1, 91, 9, 95,
  1, 6, 95, 99,
  2, 99, 10, 103,
  1, 103, 5, 107,
  2, 107, 6, 111,
  1, 111, 5, 115,
  1, 9, 115, 119,
  2, 119, 10, 123,
  1, 6, 123, 127,
  2, 13, 127, 131,
  1, 131, 6, 135,
  1, 135, 10, 139,
  1, 13, 139, 143,
  1, 143, 13, 147,
  1, 5, 147, 151,
  1, 151, 2, 155,
  1, 155, 5, 0,
  99, 2, 0, 14,
];

const DESIRED_OUTPUT: i32 = 19_690_720;

pub fn main() {
  for noun in 0..99 {
    for verb in 0..99 {
      let mut program: Vec<_> = GRAVITY_ASSIST_PROGRAM.into();
      program[1] = noun;
      program[2] = verb;
      let mut prog = Program::new(program);
      prog.run();
      let result = prog.code()[0];
      if noun == 12 && verb == 2 {
        println!("part1 solution: {}", result);
      }
      if result == DESIRED_OUTPUT {
        println!(
          "part2 solution: noun:{}, verb:{} ({})",
          noun,
          verb,
          noun * 100 + verb
        );
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run_intcode(program: impl Into<Vec<i32>>) -> Vec<i32> {
    let mut prog = Program::new(program);
    prog.run();
    prog.into_code()
  }

  #[test]
  fn example1() {
    assert_eq!(
      run_intcode(&[1, 0, 0, 0, 99] as &[i32]),
      vec![2, 0, 0, 0, 99]
    );
  }

  #[test]
  fn example2() {
    assert_eq!(
      run_intcode(&[2, 3, 0, 3, 99] as &[i32]),
      vec![2, 3, 0, 6, 99]
    );
  }

  #[test]
  fn example3() {
    assert_eq!(
      run_intcode(&[2, 4, 4, 5, 99, 0] as &[i32]),
      vec![2, 4, 4, 5, 99, 9801]
    );
  }

  #[test]
  fn example4() {
    assert_eq!(
      run_intcode(&[1, 1, 1, 4, 99, 5, 6, 0, 99] as &[i32]),
      vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
  }
}
