const INPUT: &str = include_str!("../input/day03.txt");

pub fn solve_part_1() -> u32 {
    part_1(INPUT)
}

// pub fn solve_part_2() -> i64 {
//     run_solution(INPUT, part_2)
// }

fn part_1(input: &str) -> u32 {
    input.lines().map(|line| {
        line.chars().map(|ch| {
            ch.to_digit(10)
                .expect("Invalid input! Expecting a series of lines with only digits!")
        })
    }).rfold(0, |sum, line| {
        let mut v: Vec<_> = line.collect();
        let f = v.pop().expect("Invalid input! Empty Line!");
        let (tens, ones) = v.iter().fold((0, f), |(first, snd), ch| {
            let c = *ch;
            if c > first {
                (*ch, f)
            } else if c > snd {
                (first, c)
            } else {
                (first, snd)
            }
        });
        sum + tens * 10 + ones
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let data = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

        assert_eq!(part_1(data), 357);
    }
}
