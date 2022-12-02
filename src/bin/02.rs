// Win is +6, lose is +0, draw +3
// 1 for rock -X
// 2 for paper -Y
// 3 for scissors -Z
//
// R - X
// P - Y
// S - Z
pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut left = bytes[0];
        if left == b'A' {
            left = b'X'
        } else if left == b'B' {
            left = b'Y'
        } else {
            left = b'Z'
        }

        let right = bytes[2];

        if right == b'X' {
            score += 1;
        } else if right == b'Y' {
            score += 2;
        } else {
            score += 3;
        }

        if left == right {
            score += 3;
        } else if (left == b'X' && right == b'Y')
            || (left == b'Y' && right == b'Z')
            || (left == b'Z' && right == b'X')
        {
            score += 6;
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let mut left = bytes[0];
        if left == b'A' {
            left = b'X'
        } else if left == b'B' {
            left = b'Y'
        } else {
            left = b'Z'
        }

        let right = bytes[2];
        // Rock - 1 - X
        // Paper - 2 - Y
        // Scissors - 3 - Z

        // Draw - Y
        // Lose - X
        // Win - Z

        // Draw
        if right == b'Y' {
            score += 3;

            // Rock
            if left == b'X' {
                // Rock
                score += 1;

            // Paper
            } else if left == b'Y' {
                // Paper
                score += 2;
            } else {
                score += 3;
            }
        } else if right == b'X' {
            score += 0;

            // Rock
            if left == b'X' {
                // Scissors
                score += 3;
            } else if left == b'Y' {
                score += 1;
            } else {
                score += 2;
            }
        } else {
            score += 6;

            if left == b'X' {
                score += 2;
            } else if left == b'Y' {
                score += 3;
            } else {
                score += 1;
            }
        }
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
