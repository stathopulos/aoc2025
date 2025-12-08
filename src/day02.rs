const INPUT: &str = include_str!("../input/day02.txt");

pub fn solve_part_1() -> u64 {
    run_solution(INPUT, &[2])
}

pub fn solve_part_2() -> u64 {
    // prime numbers <= 10 cover all combinations for u64 (composites are always just double counting)
    // -[6,10] to correct for composites (2*3 and 2*5) that get double counted
    // ex: 222222 will count for 2, 3, and 6 as it's all 3 repititions [222,222], [22,22,22], and [2,2,2,2,2,2]
    run_solution(INPUT, &[2, 3, 5, 7]) - run_solution(INPUT, &[6, 10])
}

const fn make_pow_10() -> [u64; 20] {
    let mut arr = [0u64; 20];

    let mut i = 0;
    while i < 20 {
        arr[i] = 10u64.pow(i as u32);
        i += 1;
    }
    arr
}
const POW10: [u64; 20] = make_pow_10();

/// Get number of digits in a u64
/// branchless binary search of the possible powers of 10 for a u64, performing a maximum of 5 comparisons
#[inline(always)]
fn len(n: u64) -> usize {
    let x: u64 = n;
    let mut res = 0;

    let shift = [10, 5, 3, 2, 1];

    for s in shift {
        let comp = (x >= POW10[res + s]) as usize; // Cast comparison to 1 or 0
        res += comp * s;
    }

    res + 1
}

/// Strategy: Repeated numbers take the form B * (10^digits - 1)/(10^size - 1)
/// where B is the number repeated, size is the number of digits in B, and digits is the number of digits total
/// we know the step size is (10^digits - 1)/(10^size - 1) so we can only need to find the highest and lowest repeated digits in our given range
/// because we have the start and end of a sequence, and the step size between each number we can sum over them cheaply
fn run_solution(input: &str, repititions: &[usize]) -> u64 {
    input
        .split(',')
        .map(|range_str| {
            let (left_str, right_str) = range_str
                .split_once("-")
                .expect("Invalid input! Not a '-' seperated range!");
            let left = left_str
                .parse::<u64>()
                .expect("Left side of range not an int");
            let right = right_str
                .trim()
                .parse::<u64>()
                .expect("Right side of range not an int");
            (left, right)
        })
        .fold(0, |acc, (left, right)| {
            let mut sum = 0;
            for &times in repititions {
                for digits in len(left)..=len(right) {
                    // skip sequences whose digits aren't divisible by the number of repeats
                    if digits % times != 0 {
                        continue;
                    }

                    // repeated numbers take the form: B * (10^digits - 1)/(10^size - 1)
                    // where B is the number repeated, size is the number of digits in B, and digits is the number of digits total
                    let block_size = (digits / times) as u32;
                    let step = (10u64.pow(digits as u32) - 1) / (10u64.pow(block_size) - 1);

                    if right < step {
                        continue;
                    }

                    let digits_min = 10u64.pow(block_size - 1);
                    let digits_max = 10u64.pow(block_size) - 1;

                    // bottom of range is either next multiple up from range min or the smallest possible repeating number for that digit
                    // which will always be the number that bisects the digits with two powers of 10: of the form `10^(n - 1) * step` ex: 11, 1010, 100100
                    let lower = left.next_multiple_of(step).max(digits_min * step);
                    // top of range is either the largest multiple < range max or the largest possible repeating number for that digit
                    // which will always be repeated 9s at every digit. This takes the form `(10^(n) - 1) * step` ex: 99, 999, 9999
                    let upper = (1 + right - step)
                        .next_multiple_of(step)
                        .min(digits_max * step);

                    if lower <= upper {
                        // arithmetic sum of (lower,lower+step...upper-step,upper)
                        let n = (upper - lower) / step + 1;
                        sum += n * (lower + upper) / 2;
                    }
                }
            }
            acc + sum
        })
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn u64_len() {
        assert_eq!(len(0), 1);
        for n in 1..9 {
            let step = (10u64.pow(n as u32) - 1) / 9;
            assert_eq!(len(step), n);
        }
    }

    #[test]
    fn part_1_example() {
        let answer = run_solution(INPUT, &[2]);

        assert_eq!(answer, 1227775554)
    }
    #[test]
    fn part_2_example() {
        let answer = run_solution(INPUT, &[2, 3, 5, 7]) - run_solution(INPUT, &[6, 10]);
        assert_eq!(answer, 4174379265)
    }
}
