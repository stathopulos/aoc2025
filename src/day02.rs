const INPUT: &str = include_str!("../input/day02.txt");

pub fn solve_part_1() -> i64 {
    run_solution(INPUT, part_1)
}

pub fn solve_part_2() -> i64 {
    run_solution(INPUT, part_2)
}

fn run_solution(input: &str, f: fn(i64) -> bool) -> i64 {
    input
        .split(",")
        .map(|range_str| {
            let (left, right) = range_str
                .split_once("-")
                .expect("Invalid input! Not a '-' seperated range!");
            let range = left.parse::<i64>().expect("Left side of range not an int")
                ..=right
                    .trim()
                    .parse::<i64>()
                    .expect("Right side of range not an int");
            range.fold(0, |acc, num| if f(num) { acc + num } else { acc })
        })
        .sum()
}

fn digits(mut i: i64) -> Vec<u8> {
    let mut digits = Vec::with_capacity(19);
    while i > 0 {
        let n = (i % 10) as u8;
        i /= 10;
        digits.push(n)
    }
    digits
}

fn part_1(i: i64) -> bool {
    let digits = digits(i);

    let (left, right) = digits.split_at(digits.len() / 2);
    *left == *right
}

fn part_2(i: i64) -> bool {
    let digits: Vec<u8> = digits(i);
    let len = digits.len();

    (1..=len / 2)
        .any(|size| len % size == 0 && digits.chunks(size).all(|chunk| chunk == &digits[0..size]))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_1_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let answer = run_solution(input, part_1);
        assert_eq!(answer, 1227775554)
    }
    #[test]
    fn part_2_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let answer = run_solution(input, part_2);
        assert_eq!(answer, 4174379265)
    }
}
