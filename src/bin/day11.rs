use advent19::Program;

#[rustfmt::skip]
const CODE: &[i64] = &[
  3, 8, 1005, 8, 302, 1106, 0, 11, 0, 0, 0, 104, 1, 104, 0, 3, 8, 102, -1, 8, 10, 101, 1, 10, 10, 
  4, 10, 1008, 8, 0, 10, 4, 10, 101, 0, 8, 29, 1006, 0, 78, 2, 1007, 9, 10, 3, 8, 1002, 8, -1, 10, 
  1001, 10, 1, 10, 4, 10, 1008, 8, 1, 10, 4, 10, 1002, 8, 1, 58, 1006, 0, 7, 3, 8, 1002, 8, -1, 10,
  101, 1, 10, 10, 4, 10, 1008, 8, 0, 10, 4, 10, 1002, 8, 1, 83, 2, 1009, 4, 10, 3, 8, 102, -1, 8, 
  10, 1001, 10, 1, 10, 4, 10, 1008, 8, 0, 10, 4, 10, 1002, 8, 1, 109, 1, 106, 11, 10, 1006, 0, 16, 
  3, 8, 1002, 8, -1, 10, 1001, 10, 1, 10, 4, 10, 1008, 8, 1, 10, 4, 10, 102, 1, 8, 138, 2, 108, 0, 
  10, 1, 101, 14, 10, 1, 1109, 1, 10, 3, 8, 1002, 8, -1, 10, 101, 1, 10, 10, 4, 10, 1008, 8, 0, 10,
  4, 10, 102, 1, 8, 172, 2, 3, 10, 10, 1006, 0, 49, 3, 8, 1002, 8, -1, 10, 101, 1, 10, 10, 4, 10, 
  1008, 8, 1, 10, 4, 10, 1001, 8, 0, 201, 1006, 0, 28, 2, 3, 15, 10, 2, 109, 12, 10, 3, 8, 1002, 8,
  -1, 10, 1001, 10, 1, 10, 4, 10, 108, 0, 8, 10, 4, 10, 1001, 8, 0, 233, 3, 8, 102, -1, 8, 10, 
  1001, 10, 1, 10, 4, 10, 108, 1, 8, 10, 4, 10, 101, 0, 8, 255, 3, 8, 1002, 8, -1, 10, 1001, 10, 1,
  10, 4, 10, 108, 1, 8, 10, 4, 10, 102, 1, 8, 277, 2, 1107, 9, 10, 101, 1, 9, 9, 1007, 9, 946, 10,
  1005, 10, 15, 99, 109, 624, 104, 0, 104, 1, 21101, 0, 932_856_042_280, 1, 21101, 0, 319, 0, 
  1105, 1, 423, 21101, 0, 387_512_640_296, 1, 21101, 330, 0, 0, 1106, 0, 423, 3, 10, 104, 0, 104, 
  1, 3, 10, 104, 0, 104, 0, 3, 10, 104, 0, 104, 1, 3, 10, 104, 0, 104, 1, 3, 10, 104, 0, 104, 0, 3,
  10, 104, 0, 104, 1, 21101, 0, 46_266_346_499, 1, 21102, 1, 377, 0, 1105, 1, 423, 21102, 1, 
  46_211_836_967, 1, 21102, 1, 388, 0, 1105, 1, 423, 3, 10, 104, 0, 104, 0, 3, 10, 104, 0, 104, 0, 
  21102, 1, 825_460_941_588, 1, 21102, 411, 1, 0, 1106, 0, 423, 21101, 709_475_738_388, 0, 1, 
  21102, 1, 422, 0, 1105, 1, 423, 99, 109, 2, 21201, -1, 0, 1, 21101, 0, 40, 2, 21102, 454, 1, 3, 
  21101, 0, 444, 0, 1106, 0, 487, 109, -2, 2106, 0, 0, 0, 1, 0, 0, 1, 109, 2, 3, 10, 204, -1, 1001,
  449, 450, 465, 4, 0, 1001, 449, 1, 449, 108, 4, 449, 10, 1006, 10, 481, 1102, 1, 0, 449, 109, 
  -2, 2106, 0, 0, 0, 109, 4, 2102, 1, -1, 486, 1207, -3, 0, 10, 1006, 10, 504, 21101, 0, 0, -3, 
  22101, 0, -3, 1, 21201, -2, 0, 2, 21102, 1, 1, 3, 21102, 1, 523, 0, 1105, 1, 528, 109, -4, 2105, 
  1, 0, 109, 5, 1207, -3, 1, 10, 1006, 10, 551, 2207, -4, -2, 10, 1006, 10, 551, 22101, 0, -4, -4, 
  1105, 1, 619, 22102, 1, -4, 1, 21201, -3, -1, 2, 21202, -2, 2, 3, 21101, 570, 0, 0, 1106, 0, 528,
  22102, 1, 1, -4, 21102, 1, 1, -1, 2207, -4, -2, 10, 1006, 10, 589, 21101, 0, 0, -1, 22202, -2, 
  -1, -2, 2107, 0, -3, 10, 1006, 10, 611, 21201, -1, 0, 1, 21101, 611, 0, 0, 106, 0, 486, 21202, 
  -2, -1, -2, 22201, -4, -2, -4, 109, -5, 2105, 1, 0
];

fn main() {
  let mut computer = Program::new(CODE);

  let mut pos = (0i64, 0i64);
  let mut painted = vec![(pos, true)];
  let mut dir = 0;
  let mut min = (0, 0);
  let mut max = (0, 0);

  loop {
    let panel = painted.binary_search_by_key(&pos, |(p, _)| *p);

    let is_painted = panel.map(|i| painted[i].1).unwrap_or(false);
    let out = match computer.run(std::iter::once(is_painted as i64)) {
      None => break,
      Some(o) => o,
    };
    match (out, panel) {
      (1, Err(i)) => {
        max = (max.0.max(pos.0), max.1.max(pos.1));
        min = (min.0.min(pos.0), min.1.min(pos.1));
        painted.insert(i, (pos, true));
      }
      (1, Ok(i)) => {
        painted[i].1 = true;
      }
      (0, Ok(i)) => {
        painted[i].1 = false;
      }
      _ => {},
    }

    let turn = computer.run(std::iter::empty()).unwrap();
    dir = if turn == 1 { dir + 1 } else { dir + 3 } % 4;
    pos = match dir {
      0 => (pos.0, pos.1 + 1),
      1 => (pos.0 - 1, pos.1),
      2 => (pos.0, pos.1 - 1),
      3 => (pos.0 + 1, pos.1),
      d => unreachable!("what the dir? {}", d),
    };
  }

  let width = (max.0 - min.0 + 1) as usize;
  let height = (max.1 - min.1 + 1) as usize;
  let mut buffer: Vec<Vec<char>> =
    std::iter::repeat_with(|| std::iter::repeat(' ').take(width).collect())
      .take(height)
      .collect();

  for ((x, y), _) in painted.iter().filter(|(_, paint)| *paint) {
    buffer[(y - min.1) as usize][(x - min.0) as usize] = '\u{2588}';
  }

  for line in buffer.iter().rev() {
    let s: String = line.iter().rev().collect();
    println!("{}", s);
  }
}
