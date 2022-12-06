

pub fn part_one(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut switch_input: bool = false;
    let mut crates = Vec::new();
    crates.resize(lines[0].len()/4+1, "".to_string());


    for line in lines {
        if line == "" {
            switch_input = true;
        } else if switch_input {
            let words = line.split(" ");
            let mut command = vec![0,0,0];
            let mut i = 0;
            for word in words {
                if word.chars().all(|c| c.is_numeric()) {
                    command[i] = word.parse::<i32>().expect("pls no");
                    i += 1;
                }
            }

            for i in 0..command[0] {
                // part 1
                // let c: char = crates[command[1] as usize-1].as_bytes()[0] as char;
                // crates[command[2] as usize-1].insert_str(0, &c.to_string());
                // crates[command[1] as usize-1].remove(0);
            }
            let slice: String = crates[command[1] as usize-1][0..command[0] as usize].to_string();
            crates[command[2] as usize-1].insert_str(0, &slice);
            crates[command[1] as usize-1].replace_range(0..command[0] as usize, "");

            
            println!("{}", line);
            let mut i = 0;
            for c in &crates {
                println!("{}: {}",i.to_string(),c);
                i += 1;
            }

        } else {
            for i in 0..line.len()/4+1 {
                let c = line.as_bytes()[i*4+1] as char;
                if c != ' ' && c.is_alphabetic() {
                    crates[i].push_str(&c.to_string());
                }
            }
        }

        
        
    }

    let mut s: String = "".to_string();
    
    for c in &crates {
        s.push_str(&(c.as_bytes()[0] as char).to_string());
    }

    return Some(s);
}

pub fn part_two(input: &str) -> Option<String> {
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
