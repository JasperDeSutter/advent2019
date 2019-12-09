pub struct Permuter {
  data: [i64; 5],
  stack: [usize; 4],
}

impl Permuter {
  pub fn new(combination: [i64; 5]) -> Self {
    Permuter {
      data: combination,
      stack: <_>::default(),
    }
  }

  pub fn combination(&self) -> [i64; 5] {
    self.data
  }

  pub fn permute(&mut self) -> bool {
    for (i, s) in self.stack.iter_mut().enumerate() {
      if *s <= i {
        let j = if (i % 2) == 0 { *s } else { 0 };
        self.data.swap(j, i + 1);
        *s += 1;
        return true;
      } else {
        *s = 0;
      }
    }
    false
  }
}

impl Iterator for Permuter {
  type Item = [i64; 5];
  fn next(&mut self) -> Option<Self::Item> {
    if self.permute() {
      Some(self.combination())
    } else {
      None
    }
  }
}
