fn main() {
    
}

struct Instruction {
    direction: char,
    steps: u64,
}

impl Into for Instruction {
   fn into(self) -> T {
       
   } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = include_str!("../../input_test.txt");
        let instructions: Vec<Instruction> = input.lines().into();
        let result = 4;
        assert_eq!(result, 4);
    }
}
