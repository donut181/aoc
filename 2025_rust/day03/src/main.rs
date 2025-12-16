
fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Wynik 1: {}", input.lines().map(|line| solve1(line).parse::<i32>().unwrap()).sum::<i32>());
    println!("WYnik 2: {}", input.lines().map(|line| solve2(line).parse::<i64>().unwrap()).sum::<i64>());
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

    let mut max_nums_indexes: [usize; 12] = [0; 12];

    max_nums_indexes[0] = find_max_return_index(&numbers, 0, numbers.len()-12);
    let mut i = 1;
    while i < 12 {
        max_nums_indexes[i] = find_max_return_index(&numbers, max_nums_indexes[i-1]+1, numbers.len() - 12 + i);
        i += 1;
    }
    return String::from_iter(max_nums_indexes.iter().map(|i| numbers[*i].to_string()));
}

fn find_max_return_index(numbers: &Vec<i32>, index_start: usize, index_end_inclusive: usize) -> usize {
    let mut max_index = index_start;
    let mut i = index_start;
    while i <= index_end_inclusive {
        if numbers[i] > numbers[max_index] {max_index = i}
        i += 1;
    }
    return max_index;
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

    #[test]
    fn test2() {
        let cases: &[(&str, &str)] = &[
            ("987654321111111", "987654321111"),
            ("811111111111119", "811111111119"),
            ("234234234234278", "434234234278"),
            ("818181911112111", "888911112111"),
        ];

        for case in cases {
            assert_eq!(solve2(case.0), case.1);
        }

    }
}