#![feature(test)]
extern crate test;

macro_rules! benchmark {
    ($($day:tt), *) => {
        $(
        mod $day {
            use aoc2025::$day::{solve_part_1, solve_part_2};

            use test::Bencher;

            #[bench]
            fn part1_bench(b: &mut Bencher) {
                b.iter(|| solve_part_1());
            }

            #[bench]
            fn part2_bench(b: &mut Bencher) {
                b.iter(|| solve_part_2());
            }
        }
    )*
    };
}

benchmark!(day02, day03, day08);
