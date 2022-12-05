pub fn part_one(input: &str) -> Option<u32> {
    // Vector of stacks.
    let mut stacks = vec![
        vec!['H', 'R', 'B', 'D', 'Z', 'F', 'L', 'S'],
        vec!['T', 'B', 'M', 'Z', 'R'],
        vec!['Z', 'L', 'C', 'H', 'N', 'S'],
        vec!['S', 'C', 'F', 'J'],
        vec!['P', 'G', 'H', 'W', 'R', 'Z', 'B'],
        vec!['V', 'J', 'Z', 'G', 'D', 'N', 'M', 'T'],
        vec!['G', 'L', 'N', 'W', 'F', 'S', 'P', 'Q'],
        vec!['M', 'Z', 'R'],
        vec!['M', 'C', 'L', 'G', 'V', 'R', 'T'],
    ];

    for line in input.lines() {
        println!("{:?}", stacks);
        let command: Vec<&str> = line.split(',').collect();
        let amount = command[0].parse::<usize>().unwrap();
        let start = command[1].parse::<usize>().unwrap() - 1;
        let end = command[2].parse::<usize>().unwrap() - 1;

        println!("{} {} {}", amount, start, end);
        let len = stacks[start].len();
        let drained: Vec<_> = stacks[start].drain(len - amount..len).rev().collect();

        println!("Drained is {:?}", drained);

        for item in drained {
            println!("APpending {}", item);
            stacks[end].push(item);
        }
    }

    println!("{:?}", stacks);
    for stack in stacks {
        println!("{}", stack[stack.len() - 1] as char);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    // Vector of stacks.
    let mut stacks = vec![
        vec!['H', 'R', 'B', 'D', 'Z', 'F', 'L', 'S'],
        vec!['T', 'B', 'M', 'Z', 'R'],
        vec!['Z', 'L', 'C', 'H', 'N', 'S'],
        vec!['S', 'C', 'F', 'J'],
        vec!['P', 'G', 'H', 'W', 'R', 'Z', 'B'],
        vec!['V', 'J', 'Z', 'G', 'D', 'N', 'M', 'T'],
        vec!['G', 'L', 'N', 'W', 'F', 'S', 'P', 'Q'],
        vec!['M', 'Z', 'R'],
        vec!['M', 'C', 'L', 'G', 'V', 'R', 'T'],
    ];

    for line in input.lines() {
        println!("{:?}", stacks);
        let command: Vec<&str> = line.split(',').collect();
        let amount = command[0].parse::<usize>().unwrap();
        let start = command[1].parse::<usize>().unwrap() - 1;
        let end = command[2].parse::<usize>().unwrap() - 1;

        println!("{} {} {}", amount, start, end);
        let len = stacks[start].len();
        let drained: Vec<_> = stacks[start].drain(len - amount..len).collect();

        println!("Drained is {:?}", drained);

        for item in drained {
            println!("APpending {}", item);
            stacks[end].push(item);
        }
    }

    println!("{:?}", stacks);
    for stack in stacks {
        println!("{}", stack[stack.len() - 1] as char);
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
