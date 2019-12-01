const MODULES_MASS: &[u32] = &[
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
    let total_fuel: u32 = MODULES_MASS.iter().copied().map(mass_required_fuel).sum();
    println!("the total required fuel for all modules is: {}", total_fuel);

    let total_fuel: u32 = MODULES_MASS.iter().copied().map(required_fuel).sum();
    println!(
        "the total required fuel for all modules including fuel is: {}",
        total_fuel
    );
}

fn calculate_fuel(mass: u32) -> Option<u32> {
    mass.checked_div(3)?.checked_sub(2)
}

fn mass_required_fuel(mass: u32) -> u32 {
    calculate_fuel(mass).unwrap_or_default()
}

fn required_fuel(mass: u32) -> u32 {
    std::iter::successors(Some(mass), |m| calculate_fuel(*m))
        .skip(1)
        .sum()
}

#[cfg(test)]
mod part1 {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            mass_required_fuel(12),
            2,
            "For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2."
        )
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            mass_required_fuel(14),
            2,
            "For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2."
        )
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            mass_required_fuel(1969),
            654,
            "For a mass of 1969, the fuel required is 654."
        )
    }

    #[test]
    fn test_example_4() {
        assert_eq!(
            mass_required_fuel(100756),
            33583,
            "For a mass of 100756, the fuel required is 33583."
        )
    }

    #[test]
    fn test_zero() {
        assert_eq!(mass_required_fuel(0), 0)
    }
}

#[cfg(test)]
mod part2 {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            required_fuel(12),
            2,
            "A module of mass 14 requires 2 fuel. This fuel requires no further
            fuel (2 divided by 3 and rounded down is 0, which would call for a
            negative fuel), so the total fuel required is still just 2."
        )
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            required_fuel(1969),
            966,
            "At first, a module of mass 1969 requires 654 fuel. Then, this fuel
            requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel,
            which requires 21 fuel, which requires 5 fuel, which requires no
            further fuel. So, the total fuel required for a module of mass 1969 is
            654 + 216 + 70 + 21 + 5 = 966"
        )
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            required_fuel(100756),
            50346,
            "The fuel required by a module of mass 100756 and its fuel is:
            33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346."
        )
    }

    #[test]
    fn test_zero() {
        assert_eq!(required_fuel(0), 0)
    }
}
