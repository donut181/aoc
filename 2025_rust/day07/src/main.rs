fn main() {
    let input = include_str!("../input.txt");
    println!("Wynik 1: {}", solution1(input));
    println!("Wynik 2: {}", solution2(input));
}

fn solution1(input: &str) -> i32 {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut num_of_splits = 0;
    for i in 1..lines.len() {
        println!("{}", lines[i-1].iter().collect::<String>());
        for j in 0..lines[0].len() {
            if lines[i-1][j] == 'S' || lines[i-1][j] == '|' {
                if lines[i][j] != '^' {
                    lines[i][j] = '|'
                } else {
                    num_of_splits += 1;
                    if j > 0 {
                        lines[i][j-1] = '|'
                    }
                    if j < lines[0].len() -1 {
                        lines[i][j+1] = '|'
                    }
                }
            }
        }
    }
    num_of_splits
}

fn solution2(input: &str) -> i32 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let line_length = lines[0].len();
    let s_pos = lines[0].iter().position(|c| *c == 'S').expect("Couldn't find S");
    let mut positions = vec![s_pos];
    let mut dups = 0;
    for i in 1..lines.len() {
        let mut new_positions: Vec<usize> = Vec::new();
        for p in &positions {
            if lines[i][*p] == '.' {
                if new_positions.contains(p) {
                    dups += 1;
                } else {
                    new_positions.push(*p);
                }
            }
            if lines[i][*p] == '^' {
                if *p > 0 {
                    if new_positions.contains(&(*p - 1)) {
                        dups += 1;
                    } else {
                        new_positions.push(*p - 1);
                    }
                }
                if *p < line_length -1 {
                    if new_positions.contains(&(*p + 1)) {
                        dups += 1;
                    } else {
                        new_positions.push(*p + 1);
                    }
                }
            }
        }
        positions = new_positions;
    }
    positions.len() as i32 + dups
}

#[cfg(test)]
mod test {
    use crate::*;


    #[test]
    fn test1() {
        let input = include_str!("../input-test.txt");
        assert_eq!(solution1(input), 21);
    }

    #[test]
    fn test2() {
        let input = include_str!("../input-test.txt");
        assert_eq!(solution2(input), 40);
    }
}
