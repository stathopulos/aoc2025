use std::str::Bytes;

#[cfg(feature = "input")]
const INPUT: &str = include_str!("../input/day01.txt");

#[cfg(feature = "input")]
pub fn solve_part_1() -> usize {
    part_1(INPUT)
}

#[cfg(feature = "input")]
pub fn solve_part_2() -> i64 {
    part_2(INPUT)
}

fn parse_instruction(bytes: &mut Bytes<'_>) -> Option<i64> {
    let sign = match bytes.next()? {
        b'L' => -1,
        b'R' => 1,
        _ => return None,
    };

    let mut n = {
        let digit = (bytes.next()? - b'0') as i64;
        (digit < 10).then_some(digit)?
    };

    for byte in bytes {
        let digit = (byte - b'0') as i64;
        if digit >= 10 {
            break;
        }
        n = 10 * n + digit
    }
    Some(sign * n)
}
fn part_1(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|s| parse_instruction(&mut s.bytes()).expect("Could not parse instruction!"));
    let mut count = 0;
    let mut dial = 50;
    for turns in instructions {
        dial = (dial + turns) % (100);
        if dial == 0 {
            count += 1;
        }
    }
    count
}

fn part_2(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|s| parse_instruction(&mut s.bytes()).expect("Could not parse instruction!"));
    let mut count = 0;
    let mut dial = 50;
    for turns in instructions {
        if turns.is_negative() {
            // "reverse" the dial for easier left turns
            let rev = (100 - dial) % 100;
            count += (rev - turns) / 100
        } else {
            count += (dial + turns) / 100
        }
        dial = (dial + turns).rem_euclid(100);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn part_1_example() {
        let answer = part_1(INPUT);

        assert_eq!(answer, 3)
    }
    #[test]
    fn part_2_example() {
        let answer = part_2(INPUT);
        assert_eq!(answer, 6)
    }
}
