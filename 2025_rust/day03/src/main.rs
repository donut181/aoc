use std::env::current_exe;

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Wynik 1: {}", input.lines().map(|line| solve1(line).parse::<i32>().unwrap()).sum::<i32>());
    println!("WYnik 2: {}", input.lines().map(|line| solve1(line).parse::<i64>().unwrap()).sum::<i64>());
}

fn solve1(line: &str) -> String {
    let mut first_max = 0;
    let mut i = 1;
    let numbers: Vec<i32> = line.chars().map(|x| x.to_digit(10).unwrap() as i32 ).collect();
    while i < line.len() - 1 {
        if numbers[i] > numbers[first_max] {first_max = i}
        i +=1;
    }
    let mut second_max = first_max + 1;
    let mut i = second_max;
    while i < line.len() {
        if numbers[i] > numbers[second_max] {second_max = i}
        i += 1;
    }
    return format!("{}{}", numbers[first_max], numbers[second_max]);
}

fn solve2(line: &str) -> String {
    let numbers: Vec<i32> = line.chars().map(|x| x.to_digit(10).unwrap() as i32 ).collect();

    let mut max_nums_indexes: [u8; 12];

    let mut current_max_num_index = 0;

    while max_nums_indexes[current_max_num_index] < max_nums_indexes.len() - 1 {
        if numbers[i] > numbers[first_max] {first_max = i}
        i +=1;
    }

    }
    let mut second_max = first_max + 1;
    let mut i = second_max;
    while i < line.len() {
        if numbers[i] > numbers[second_max] {second_max = i}
        i += 1;
    }
    return format!("{}{}", numbers[first_max], numbers[second_max]);
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test1(){
        
        let cases: &[(&str, &str)] = &[
            ("987654321111111", "98"),
            ("811111111111119", "89"),
            ("234234234234278", "78"),
            ("818181911112111", "92"),
        ];

        for case in cases {
            assert_eq!(solve1(case.0), case.1);
        }

        assert_eq!(
            cases.iter().map(|x| solve1(x.0).parse::<i32>().unwrap()).sum::<i32>(), 
            357
        )
    }
}