fn main() {
    const START: usize = 265_275;
    const END: usize = 781_584;

    let matching = (START..=END)
        .filter(|i| is_valid_password(i.to_string().as_bytes()))
        .count();
    println!("amount of matching passwords: {:?}", matching);
}

fn is_valid_password(password: &[u8]) -> bool {
    let two_equals = password
        .iter()
        .zip(password.iter().skip(1))
        .any(|(l, r)| l == r);

    two_equals
        && password
            .iter()
            .scan(b'1', |st, i| {
                i.checked_sub(*st)?;
                *st = *i;
                Some(i)
            })
            .count()
            == password.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(is_valid_password(b"111111"), true);
    }

    #[test]
    fn example2() {
        assert_eq!(is_valid_password(b"223450"), false);
    }

    #[test]
    fn example3() {
        assert_eq!(is_valid_password(b"123789"), false);
    }

    #[test]
    fn explanation() {
        assert_eq!(is_valid_password(b"122345"), true);
    }
}
