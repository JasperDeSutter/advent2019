use std::env;

const MODULES_MASS: &[u64] = &[
    106404, 140515, 142745, 120767, 79665, 54235, 127391, 72207, 70799, 79485, 103994, 129583,
    132791, 95135, 121194, 129425, 64861, 123233, 132805, 87916, 111395, 126625, 113045, 61704,
    65413, 145820, 75988, 74717, 115137, 85331, 86833, 86063, 85464, 139738, 103372, 101942, 52741,
    77660, 112745, 103109, 106301, 141714, 74546, 55474, 106747, 140234, 60426, 145867, 144810,
    94179, 101606, 77763, 139291, 104246, 148513, 126828, 64624, 139058, 85839, 86636, 62198,
    137358, 76711, 87848, 141711, 114079, 71639, 95896, 104522, 61929, 72199, 142790, 137736,
    123437, 91872, 127661, 111179, 51548, 83452, 91196, 117798, 84484, 75517, 83820, 97407, 89181,
    71428, 72758, 73076, 109957, 50601, 74571, 65556, 129765, 80626, 126995, 73480, 71360, 103288,
    85670,
];

pub fn main() {
    if let Some(number) = env::args().last() {
        if let Ok(value) = number.parse::<u64>() {
            println!(
                "required fuel for mass {} is: {}",
                value,
                required_fuel(value)
            );
            return;
        }
    }

    let total_fuel: u64 = MODULES_MASS.iter().copied().map(required_fuel).sum();
    println!("the total required fuel for all modules is: {}", total_fuel);
}

pub fn required_fuel(mass: u64) -> u64 {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            required_fuel(12),
            2,
            "For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2."
        )
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            required_fuel(14),
            2,
            "For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2."
        )
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            required_fuel(1969),
            654,
            "For a mass of 1969, the fuel required is 654."
        )
    }

    #[test]
    fn test_example_4() {
        assert_eq!(
            required_fuel(100756),
            33583,
            "For a mass of 100756, the fuel required is 33583."
        )
    }
}
