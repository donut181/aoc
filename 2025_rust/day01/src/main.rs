use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    println!("Wynik to {:?}", process(input));
}

struct Instruction {
    direction: u8,
    steps: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;

impl FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction: u8 = s.as_bytes()[0];
        let steps: i32 = s[1..].parse().expect(format!("Could not parse int {s}").as_str());
        return Ok(Instruction { direction, steps })
    }
}

fn process(input: &str) -> (i32, i32) {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();
    let mut current: i32 = 50;
    let mut num_of_zeros_a = 0;
    let mut num_of_zeros_b = 0;
    println!("The dial starts by pointing at {}.", current);
    for instr in instructions {
        let mut flag = 0;
        let mut i = instr.steps;
        if instr.direction == b'L' {
            while i > 0 {
                current -= 1;
                if current == -1 {current += 100}
                if current == 0 {num_of_zeros_b += 1; flag += 1}
                i -= 1;
            }
        } else {
            while i > 0 {
                current += 1;
                if current == 100 {current -= 100}
                if current == 0 {num_of_zeros_b += 1; flag += 1}
                i -= 1;
            }
        }
        if current == 0 {
            num_of_zeros_a += 1;
            if flag != 0 {
                flag -= 1;
            }
        }
        print!("The dial is rotated {}{} to point at {}", instr.direction as char, instr.steps, current);
        if flag != 0 {print!("; during this rotation, it points at 0 {} times", flag)}
        println!()
    }
    return (num_of_zeros_a, num_of_zeros_b);
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(process(input), (3, 6))
    }
}
