const INPUT: &str = include_str!("../input/day03.txt");

const LINE_SIZE: usize = {
    let bytes = INPUT.as_bytes();
    let mut len = 0;
    while len < bytes.len() && bytes[len] != b'\n' {
        len += 1
    }
    len
};

pub fn solve_part_1() -> u32 {
    part_1(INPUT)
}

pub fn solve_part_2() -> u64 {
    part_2(INPUT, LINE_SIZE)
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars().map(|ch| {
                ch.to_digit(10)
                    .expect("Invalid input! Expecting a series of lines with only digits!")
            })
        })
        .fold(0, |sum, line| {
            let mut digits: Vec<_> = line.collect();
            let last_digit: u32 = digits.pop().expect("Invalid input! Empty Line!");
            let (tens, ones) = digits.iter().fold((0, last_digit), |(first, snd), ch| {
                let c = *ch;
                if c > first {
                    (*ch, last_digit)
                } else if c > snd {
                    (first, c)
                } else {
                    (first, snd)
                }
            });
            sum + tens * 10 + ones
        })
}

fn part_2(input: &str, line_size: usize) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars().map(|ch| {
                ch.to_digit(10)
                    .expect("Invalid input! Expecting a series of lines with only digits!")
            })
        })
        .fold(0, |sum, line| {
            let mut to_remove = line_size - 12;
            let mut stack = Vec::with_capacity(12);

            for digit in line {
                while let Some(&last) = stack.last() {
                    if to_remove > 0 && last < digit {
                        stack.pop();
                        to_remove -= 1;
                    } else {
                        break;
                    }
                }
                stack.push(digit);
            }
            stack.truncate(12);

            sum + stack
                .into_iter()
                .fold(0, |sum, digit| sum * 10 + digit as u64)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(DATA), 357);
    }

    #[test]
    fn part_2_example() {
        const TEST_SIZE: usize = {
            let bytes = DATA.as_bytes();
            let mut len = 0;
            while len < bytes.len() && bytes[len] != b'\n' {
                len += 1
            }
            len
        };
        assert_eq!(part_2(DATA, TEST_SIZE), 3121910778619);
    }
}
