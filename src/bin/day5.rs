use advent19::run_intcode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            run_intcode(vec![1002, 4, 3, 4, 33], 0),
            vec![1002, 4, 3, 4, 99]
        );
    }
}
