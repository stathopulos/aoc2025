const INPUT: &str = include_str!("../input/day03.txt");

pub fn solve_part_1() -> u32 {
    part_1(INPUT)
}

// pub fn solve_part_2() -> i64 {
//     run_solution(INPUT, part_2)
// }

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("Invalid input!"))
        })
        .fold(0, |sum, line| {
            let v: Vec<_> = line.collect();
            let (tens, ones) = v.iter().fold((0, 0), |(first, snd), ch| {
                let c = *ch;
                if c > first {
                    (*ch, 0)
                } else if c > snd {
                    (first, c)
                } else {
                    (first, snd)
                }
            });
            if ones == 0 {
                let new_ones = v
                    .into_iter()
                    .skip_while(|c| *c != tens - 1)
                    .fold(0, |ones, ch| if ch > ones { ch } else { ones });
                sum + 10 * (tens - 1) + new_ones
            } else {
                sum + tens * 10 + ones
            }
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
