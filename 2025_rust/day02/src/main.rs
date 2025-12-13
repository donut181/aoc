fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Wynik: {:?}", (process(input, invalid_ids_a), process(input, invalid_ids_b)))
}

fn invalid_ids_a(range: &str) -> Vec<i64> {
    let (start, stop) = range.split_once('-').expect(&format!("Couldn't split on '-' in {}", range));
    let start: i64 = start.parse().expect(&format!("Couldn't parse start {}..{}", start, stop));
    let stop: i64 = stop.parse().expect(&format!("Couldn't parse stop {}..{}", start, stop));
    let mut result: Vec<i64> = Vec::new();

    for num in start..=stop {
        let id: String = num.to_string();
        if id.len().is_multiple_of(2) {
            let middle = id.len() / 2;
            if id[0..middle] == id[middle..id.len()] {
                result.push(num);
            }
        }
    }

    result
}

fn invalid_ids_b(range: &str) -> Vec<i64> {
    let (start, stop) = range.split_once('-').expect(&format!("Couldn't split on '-' in {}", range));
    let start: i64 = start.parse().expect(&format!("Couldn't parse start {}..{}", start, stop));
    let stop: i64 = stop.parse().expect(&format!("Couldn't parse stop {}..{}", start, stop));
    let mut result: Vec<i64> = Vec::new();

    for num in start..=stop {
        let id: String = num.to_string();
        let limit = id.len()/2; // 2 for len == 5

        'given_num: for i in 1..=limit {
            // let's check for substring of len i

            if id.len().is_multiple_of(i) {
                // if id len is not multiple of i it cannot work

                let mut valid = true;

                let occurances = id.len() / i;
                for j in 1..occurances {
                    if id[0..i] != id[i*j..i*j+i] {
                        valid = false;
                        break
                    }
                }

                if valid {
                    result.push(num);
                    break 'given_num;
                }

            }
        }

    }

    result
}

fn process(input: &str, invalid_ids: fn(&str)->Vec<i64>) -> i64 {
    let mut invalid_nums: Vec<i64> = Vec::new();
    input.split(',').for_each(|range| {
        let mut ids = invalid_ids(range);
        println!("{} has invalid IDs: {:?}", range, &ids);
        invalid_nums.append(&mut ids);
    });

    return invalid_nums.iter().sum();
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        let input = include_str!("../input_test.txt").trim();
        assert_eq!(process(input, invalid_ids_a), 1227775554);
    }

    #[test]
    fn test_a() {
        let cases: [(&str, Vec<i64>);_]= [
            ("11-22", vec![11, 22]),
            ("95-115", vec![99]),
            ("998-1012", vec![1010]),
            ("1188511880-1188511890", vec![1188511885]),
            ("222220-222224", vec![222222]),
            ("1698522-1698528", vec![]),
            ("446443-446449", vec![446446]),
            ("38593856-38593862", vec![38593859]),
        ];
        for case in cases {
            assert_eq!(invalid_ids_a(case.0), case.1)
        }
    }

    #[test]
    fn test_b() {
        let cases: [(&str, Vec<i64>);_]= [
            ("11-22", vec![11, 22]),
            ("95-115", vec![99, 111]),
            ("998-1012", vec![999, 1010]),
            ("1188511880-1188511890", vec![1188511885]),
            ("222220-222224", vec![222222]),
            ("1698522-1698528", vec![]),
            ("446443-446449", vec![446446]),
            ("38593856-38593862", vec![38593859]),
            ("565653-565659", vec![565656]),
            ("824824821-824824827", vec![824824824]),
            ("2121212118-2121212124", vec![2121212121]),
        ];
        for case in cases {
            assert_eq!(invalid_ids_b(case.0), case.1)
        }
    }

    #[test]
    fn testtttt(){
        assert_eq!(5/2, 2)
    }
}