use advent19::Program;

const AMPLIFIER_CODE: &[i32] = &[
  3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 42, 67, 88, 101, 114, 195, 276, 357, 438, 99999, 3, 9,
  101, 3, 9, 9, 1002, 9, 4, 9, 1001, 9, 5, 9, 102, 4, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 3, 9, 1002, 9,
  2, 9, 101, 2, 9, 9, 102, 2, 9, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 102, 4, 9, 9, 1001, 9, 3, 9,
  102, 4, 9, 9, 101, 4, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 101, 4,
  9, 9, 1002, 9, 5, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1,
  9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
  101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4,
  9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
  1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9,
  4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9,
  102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
  9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1,
  9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3,
  9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9,
  4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002,
  9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4,
  9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002,
  9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3,
  9, 101, 2, 9, 9, 4, 9, 99,
];

fn get_max_truster_value(code: &[i32]) -> (i32, [i32; 5]) {
  let mut max = <(i32, [i32; 5])>::default();

  let mut combination = [0, 1, 2, 3, 4];
  let mut stack = [0usize; 4];

  'outer: loop {
    let thruster_signal = combination
      .iter()
      .scan(0, |signal, &phase| {
        let input = [phase, *signal];
        let mut input_iter = input.iter();
        let mut program = Program::new(code);
        *signal = program.run(|| *input_iter.next().unwrap()).unwrap();
        Some(*signal)
      })
      .last()
      .unwrap();

    if thruster_signal > max.0 {
      max = (thruster_signal, combination)
    }

    for (i, s) in stack.iter_mut().enumerate() {
      if *s <= i {
        let j = if (i % 2) == 0 { *s } else { 0 };
        combination.swap(j, i + 1);
        *s += 1;
        continue 'outer;
      } else {
        *s = 0;
      }
    }

    break;
  }

  max
}

fn max_truster_value_feedback(code: &[i32]) -> (i32, [i32; 5]) {
  let mut max = <(i32, [i32; 5])>::default();

  let mut combination = [5, 6, 7, 8, 9];
  let mut stack = [0usize; 4];

  'outer: loop {
    let mut programs = [
      Program::new(code),
      Program::new(code),
      Program::new(code),
      Program::new(code),
      Program::new(code),
    ];

    let mut signal = 0;
    for i in 0..5 {
      let input = [combination[i], signal];
      let mut input_iter = input.iter();
      signal = programs[i].run(|| *input_iter.next().unwrap()).unwrap();
    }
    let mut i = 0;
    while let Some(result) = programs[i].run(|| signal) {
      signal = result;
      i = (i + 1) % 5;
    }

    if signal > max.0 {
      max = (signal, combination)
    }

    for (i, s) in stack.iter_mut().enumerate() {
      if *s <= i {
        let j = if (i % 2) == 0 { *s } else { 0 };
        combination.swap(j, i + 1);
        *s += 1;
        continue 'outer;
      } else {
        *s = 0;
      }
    }

    break;
  }

  max
}

fn main() {
  let result = get_max_truster_value(&AMPLIFIER_CODE);
  println!("thruster signal: {:?}", result);
  let result = max_truster_value_feedback(&AMPLIFIER_CODE);
  println!("thruster signal + feedback: {:?}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example1() {
    assert_eq!(
      get_max_truster_value(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
      (43210, [4, 3, 2, 1, 0])
    )
  }

  #[test]
  fn example2() {
    assert_eq!(
      get_max_truster_value(&[
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0
      ]),
      (54321, [0, 1, 2, 3, 4])
    )
  }

  #[test]
  fn example3() {
    assert_eq!(
      get_max_truster_value(&[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
      ]),
      (65210, [1, 0, 4, 3, 2])
    )
  }

  #[test]
  fn example4() {
    assert_eq!(
      max_truster_value_feedback(&[
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5
      ]),
      (139_629_729, [9, 8, 7, 6, 5])
    )
  }

  #[test]
  fn example5() {
    assert_eq!(
      max_truster_value_feedback(&[
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
      ]),
      (18216, [9, 7, 8, 5, 6])
    )
  }
}
