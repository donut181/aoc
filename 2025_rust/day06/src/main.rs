fn main() {
    let input = include_str!("../input.txt");
    println!("Wynik 1: {}", solve1(input));
}

fn solve1(input: &str) -> i64 {
    let mut rows: Vec<Vec<&str>>  = input.lines().map(|l| l.trim()).map(|l| l.split_whitespace().collect()).collect();
    assert!(rows.windows(2).all(|w| w[0].len() == w[1].len()));
    let ops = rows.pop().expect("no data in rows");
    let mut columns:Vec<Vec<i64>> = Vec::new();
    for column in 0..ops.len() {
        let mut clmn:Vec<i64> = Vec::new();
        for row in 0..rows.len() {
            clmn.push(rows[row][column].parse().expect("Couldn't parse"));
        }
        columns.push(clmn);
    }
    let mut result: Vec<i64> = Vec::new();
    for (i, c) in columns.iter().enumerate() {
        result.push(match ops[i] {
            "+" => c.iter().sum(),
            "*" => c.iter().product(),
            _ => panic!("Unknown op {}", ops[i]),
        });
    }
    result.iter().sum()
}

fn solve2(input: &str) -> i64 {
    let data:Vec<&str> = input.lines().collect();
    let len = data[0].len();
    let height = data.len();
    let mut current_nums: Vec<i64> = Vec::new();
    let mut current_op = '+';
    for c in len-1..0 {
        let mut column: Vec<char> = Vec::new();
        match data[height-1].chars()[c] {
            ' ' => 
        }
        for i in 0..height-1 {
            match data[i].chars().nth(c).expect("Coulnd't get nth char") {
                ' ' => continue,
                i => column.push(i),
                _ => panic!("Shouldn't happen"),
            }
            if column.len() > 0 {
                current_nums.push(column.iter().collect::<String>().parse::<i64>().expect("Couldn't parse i64"));
            }

        }
    }
    1234
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = include_str!("../input-test.txt");
        assert_eq!(
            solve1(input), 
            4277556
        );
    }

    #[test]
    fn test2() {
        let input = include_str!("../input-test.txt");
        assert_eq!(
            solve2(input),
            14
        )
    }
}