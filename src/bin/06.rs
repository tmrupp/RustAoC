fn all_unique (input: &String) -> bool {
    let i = 0;
    for c in input.chars() {
        let mut xs = input.clone();
        xs.remove(i);

        if xs.contains(c) {
            return false;
        }

    }

    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut xs = "".to_string();

    let mut i = 0;
    for c in input.chars() {
        if xs.len() == 4 {
            if all_unique(&xs) {
                return Some(i);
            }
            xs.remove(0);
        }

        xs.push(c);
        println!("{}", xs);
        i += 1;
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
