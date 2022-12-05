pub fn part_one(input: &str) -> Option<u32> {
    let mut contained = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let split_left: Vec<&str> = split[0].split('-').collect();
        let left_low = split_left[0].parse::<u64>().unwrap();
        let left_high = split_left[1].parse::<u64>().unwrap();

        let split_right: Vec<&str> = split[1].split('-').collect();
        let right_low = split_right[0].parse::<u64>().unwrap();
        let right_high = split_right[1].parse::<u64>().unwrap();

        if left_low >= right_low && left_high <= right_high
            || right_low >= left_low && right_high <= left_high
        {
            contained += 1
        }
    }
    println!("{:?}", contained);

    Some(contained)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut contained = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let split_left: Vec<&str> = split[0].split('-').collect();
        let left_low = split_left[0].parse::<u64>().unwrap();
        let left_high = split_left[1].parse::<u64>().unwrap();

        let split_right: Vec<&str> = split[1].split('-').collect();
        let right_low = split_right[0].parse::<u64>().unwrap();
        let right_high = split_right[1].parse::<u64>().unwrap();

        // If the higher bound on the left, is greater than lower bound on the right.
        //
        // Start of one range is below end of another range.
        //
        // r1 <= l2
        // l1 <= r2
        if left_low <= right_high && right_low <= left_high {
            contained += 1;
        };
    }

    println!("{:?}", contained);

    Some(contained)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(305));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(811));
    }
}
