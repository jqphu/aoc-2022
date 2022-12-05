use itertools::Itertools;
pub fn part_one(input: &str) -> Option<u32> {
    let mut found: Vec<u8> = vec![];
    for line in input.lines() {
        let mut bytes: Vec<u8> = line.as_bytes().into();

        let mid = bytes.len() / 2;

        let (left, right) = bytes.split_at_mut(mid);

        'outer: for i in left {
            for r in &mut *right {
                if i == r {
                    found.push(*i);
                    break 'outer;
                }
            }
        }
    }

    let mut total = 0_u32;

    for i in found {
        if (b'a'..=b'z').contains(&i) {
            total += (i - b'a' + 1) as u32;
        } else {
            total += (i - b'A' + 26 + 1) as u32;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut found: Vec<u8> = vec![];
    for lines in &input.lines().chunks(3) {
        let lines = lines.collect_vec();
        let a = lines[0].as_bytes();
        let b = lines[1].as_bytes();
        let c = lines[2].as_bytes();

        'outer: for i in a {
            for j in b {
                for k in c {
                    if i == j && j == k {
                        found.push(*i);
                        break 'outer;
                    }
                }
            }
        }
    }
    let mut total = 0_u32;
    for i in found {
        if (b'a'..=b'z').contains(&i) {
            total += (i - b'a' + 1) as u32;
        } else {
            total += (i - b'A' + 26 + 1) as u32;
        }
    }

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(7568));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(2780));
    }
}
