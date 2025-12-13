#[cfg(feature = "input")]
const INPUT: &str = include_str!("../input/day08.txt");

use std::collections::BTreeSet;
use std::collections::HashSet;

use vec3::Vec3;

#[cfg(feature = "input")]
pub fn solve_part_1() -> i32 {
    parse(INPUT, 1000)
}

#[cfg(feature = "input")]
pub fn solve_part_2() -> u64 {
    parse_last_two(INPUT, 1000)
}

mod vec3 {
    use std::num::ParseIntError;
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct Vec3(u64, u64, u64);

    impl Vec3 {
        pub fn x(&self) -> u64 {
            self.0
        }
        pub fn y(&self) -> u64 {
            self.1
        }
        pub fn z(&self) -> u64 {
            self.2
        }
        pub fn sq_dist(&self, other: &Vec3) -> u64 {
            (self.x().abs_diff(other.x())).pow(2)
                + (self.y().abs_diff(other.y())).pow(2)
                + (self.z().abs_diff(other.z())).pow(2)
        }
    }

    impl FromStr for Vec3 {
        type Err = TryFromStrError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut split = s.split(',');
            match (split.next(), split.next(), split.next(), split.next()) {
                (None, _, _, _) | (_, None, _, _) | (_, _, None, _) | (_, _, _, Some(_)) => {
                    Err(TryFromStrError::Not3CommaSeperatedValues)
                }
                (Some(x), Some(y), Some(z), _) => Ok(Vec3(x.parse()?, y.parse()?, z.parse()?)),
            }
        }
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum TryFromStrError {
        Not3CommaSeperatedValues,
        Not3Ints(ParseIntError),
    }

    impl From<ParseIntError> for TryFromStrError {
        fn from(value: ParseIntError) -> Self {
            Self::Not3Ints(value)
        }
    }
}

#[derive(Debug)]
struct VecPair(Vec3, Vec3);

impl VecPair {
    fn to_pair(&self) -> (Vec3, Vec3) {
        (self.0, self.1)
    }
    fn sq_dist(&self) -> u64 {
        self.0.sq_dist(&self.1)
    }
}

// order by square distance
impl Ord for VecPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sq_dist().cmp(&other.sq_dist())
    }
}

impl PartialOrd for VecPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// compare equality order agnostic to match distance based Ord and ignore duplicates
impl PartialEq for VecPair {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for VecPair {}

#[derive(Debug, PartialEq, Eq)]
struct Circuit {
    nodes: HashSet<Vec3>,
}

// order by size of each circuit, starting with the largest
impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.nodes.len().cmp(&other.nodes.len()).reverse()
    }
}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> BTreeSet<VecPair> {
    let lines = input.lines();
    // a set of each point paired with every other point, brute force and slow, but not that bad given our n
    lines
        .clone()
        .flat_map(|l| {
            lines.clone().filter(move |&r| l != r).map(|r| {
                let left = l.parse::<Vec3>().expect("Malformed input!");
                let right = r.parse::<Vec3>().expect("Malformed input!");
                VecPair(left, right)
            })
        })
        .collect()
}

fn parse(input: &str, num_conns: usize) -> i32 {
    let pairs = parse_input(input);
    let mut circuits: Vec<Circuit> = Vec::new();
    for v in pairs.into_iter().take(num_conns) {
        let (left, right) = v.to_pair();
        let encountered = circuits
            .extract_if(.., |c| c.nodes.contains(&left) || c.nodes.contains(&right))
            .fold(HashSet::from([left, right]), |acc, c| {
                c.nodes.union(&acc).copied().collect()
            });

        circuits.push(Circuit { nodes: encountered });
    }

    circuits.sort();
    circuits
        .iter()
        .take(3)
        .map(|c| c.nodes.len() as i32)
        .product()
}

fn parse_last_two(input: &str, input_len: usize) -> u64 {
    let mut pairs = parse_input(input);
    let mut circuits: Vec<Circuit> = Vec::new();
    let (left, right) = loop {
        let v = pairs.pop_first().unwrap();
        let (left, right) = v.to_pair();
        let encountered = circuits
            .extract_if(.., |c| c.nodes.contains(&left) || c.nodes.contains(&right))
            .fold(HashSet::from([left, right]), |acc, c| {
                c.nodes.union(&acc).copied().collect()
            });

        circuits.push(Circuit { nodes: encountered });
        let i = circuits.iter().map(|c| c.nodes.len()).sum::<usize>();
        if i >= input_len {
            break v.to_pair();
        }
    };

    left.x() * right.x()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_1_example() {
        const INPUT: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

        let answer = parse(INPUT, 10);
        assert_eq!(answer, 40)
    }

    #[test]
    fn part_2_example() {
        const INPUT: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

        let answer = parse_last_two(INPUT, 20);
        assert_eq!(answer, 25272)
    }
}
