use std::{array, mem};

#[cfg(feature = "input")]
const INPUT: &str = include_str!("../input/day04.txt");

const INPUT_SIZE: usize = {
    let bytes = INPUT.as_bytes();
    let mut len = 0;
    while len < bytes.len() && bytes[len] != b'\n' {
        len += 1
    }
    len
};

#[cfg(feature = "input")]
pub fn solve_part_1() -> usize {
    part_1::<INPUT_SIZE>(INPUT)
}

#[cfg(feature = "input")]
pub fn solve_part_2() -> usize {
    part_2::<INPUT_SIZE>(INPUT)
}

#[derive(Debug)]
struct Grid<const N: usize>([[u8; N]; N]);

impl<const N: usize> Grid<N> {
    fn is_unblocked(&self, idx_line: usize, idx_byte: usize) -> bool {
        let top = idx_line.checked_sub(1);
        let left = idx_byte.checked_sub(1);
        let bottom = (idx_line + 1 < N).then_some(idx_line + 1);
        let right: Option<usize> = (idx_byte + 1 < N).then_some(idx_byte + 1);

        let mut count = 0;
        for maybe_i in [top, Some(idx_line), bottom] {
            for maybe_j in [left, Some(idx_byte), right] {
                if let (Some(i), Some(j)) = (maybe_i, maybe_j)
                    && self.0[i][j] == b'@'
                {
                    count += 1;
                }
            }
        }
        count < 5
    }
}

fn remove_accessible<const N: usize>(grid: &mut Grid<N>) -> usize {
    let mut count = 0;
    for i in 0..N {
        for j in 0..N {
            if grid.0[i][j] == b'@' && grid.is_unblocked(i, j) {
                count += 1;
                let _ = mem::replace(&mut grid.0[i][j], b'.');
            }
        }
    }
    count
}

fn parse<const N: usize>(input: &str) -> Grid<N> {
    let mut lines = input.lines().map(|s| s.as_bytes());
    Grid(array::from_fn(|_| {
        lines.next().unwrap().try_into().unwrap()
    }))
}

fn part_1<const SIZE: usize>(input: &str) -> usize {
    let grid: Grid<SIZE> = parse(input);
    let mut count = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if grid.0[i][j] == b'@' && grid.is_unblocked(i, j) {
                count += 1
            }
        }
    }
    count
}

fn part_2<const SIZE: usize>(input: &str) -> usize {
    let mut grid: Grid<SIZE> = parse(input);
    let mut count = 0;
    let mut res = remove_accessible(&mut grid);
    while res != 0 {
        count += res;
        res = remove_accessible(&mut grid);
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";

    const DATA_SIZE: usize = {
        let bytes = DATA.as_bytes();
        let mut len = 0;
        while len < bytes.len() && bytes[len] != b'\n' {
            len += 1
        }
        len
    };

    #[test]
    fn part_1_example() {
        let answer = part_1::<DATA_SIZE>(DATA);

        assert_eq!(answer, 13)
    }

    #[test]
    fn part_2_example() {
        let answer = part_2::<DATA_SIZE>(DATA);

        assert_eq!(answer, 43)
    }
}
