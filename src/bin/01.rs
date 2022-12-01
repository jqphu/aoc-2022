pub fn part_one(input: &str) -> Option<u32> {
    let mut calories = vec![];
    let mut counter = 0;
    for line in input.lines() {
        if !line.is_empty() {
            counter += line.parse::<u32>().unwrap();
        } else {
            calories.push(counter);
            counter = 0;
        }
    }

    calories.push(counter);

    calories.sort_by(|a, b| b.cmp(a));
    Some(calories[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = vec![];
    let mut counter = 0;
    for line in input.lines() {
        if !line.is_empty() {
            counter += line.parse::<u32>().unwrap();
        } else {
            calories.push(counter);
            counter = 0;
        }
    }

    calories.push(counter);

    calories.sort_by(|a, b| b.cmp(a));
    Some(calories[0] + calories[1] + calories[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(70296));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(205381));
    }
}
