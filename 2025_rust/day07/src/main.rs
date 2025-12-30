fn main() {
    let input = include_str!("../input.txt");
    println!("Wynik 1: {}", solution1(input));
    println!("Wynik 2: {}", solution2(input));
}

fn solution1(input: &str) -> i32 {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut num_of_splits = 0;
    for i in 1..lines.len() {
        // println!("{}", lines[i-1].iter().collect::<String>());
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

fn solution2(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let line_length = lines[0].len();
    let s_pos = lines[0].iter().position(|c| *c == 'S').expect("Couldn't find S");
    let mut num_of_photons: Vec<u64>= vec![0; line_length];
    num_of_photons[s_pos] += 1;
    for i in 1..lines.len() {
        let mut new_num_of_photons: Vec<u64> = vec![0; line_length];
        for (j, num) in num_of_photons.iter().enumerate() {
            if *num > 0 {
                if lines[i][j] == '.' {
                    new_num_of_photons[j] += *num;
                }
                if lines[i][j] == '^' {
                    if j > 0 {
                        new_num_of_photons[j - 1] += *num;
                    }
                    if j < line_length -1 {
                        new_num_of_photons[j + 1] += *num;
                    }
                }
            }
        }
        num_of_photons = new_num_of_photons;
    }
    num_of_photons.iter().sum()
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
