#[cfg(feature = "input")]
const INPUT: &str = include_str!("../input/day05.txt");

#[cfg(feature = "input")]
pub fn solve_part_1() -> usize {
    part_1(INPUT)
}

fn part_1(input: &str) -> usize {
    let (left, right) = input
        .split_once("\n\n")
        .expect(r#"Input is not ranges and integers seperated by \n\n"#);
    let ranges: Vec<_> = left
        .lines()
        .map(|range_str| {
            let (left_str, right_str) = range_str
                .split_once("-")
                .expect("Invalid input! Not a '-' seperated range!");
            let left = left_str
                .parse::<u64>()
                .expect("Left side of range not an int");
            let right = right_str
                .parse::<u64>()
                .expect("Right side of range not an int");
            left..=right
        })
        .collect();
    let mut count = 0;
    for s in right.lines() {
        let i: u64 = s
            .parse()
            .expect("Failed to parse integer in second section!");
        if ranges.iter().any(|range| range.contains(&i)) {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn part_1_example() {
        let answer = part_1(DATA);

        assert_eq!(answer, 3)
    }
}
