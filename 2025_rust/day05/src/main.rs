fn main() {
    let input = include_str!("../input.txt");
    println!("Wynik 1: {}", solve1(input));
    println!("Wynik 2: {}", solve2(input));
}

#[derive(PartialEq, Debug)]
struct Range {
    beginning: i64,
    end: i64,
}
impl Range {
    fn new(range_input: &str) -> Self {
        let (b, e) = range_input
            .split_once("-")
            .and_then(|r| 
                Some((
                    r.0.parse::<i64>().expect("Coulnd't parse beginning of a range"),
                    r.1.parse::<i64>().expect("Couldn't parse end of a range")
                ))
            ).expect("Couldn't parse a range");
        Range { beginning: b, end: e }
    }

    fn contains(&self, i: i64) -> bool {
        i >= self.beginning && i <= self.end
    }

    fn span(&self) -> i64 {
        self.end - self.beginning + 1
    }

    fn merge(&self, other: &Range) -> Option<Range> {
        // disjoined
        if other.end < self.beginning { return None }
        if other.beginning > self.end { return None }

        Some(
            Range{
                beginning: self.beginning.min(other.beginning),
                end: self.end.max(other.end)
            }
        )
    }
}

fn solve1(input: &str) -> i32 {
    let (ranges, ingredients) = input.split_once("\n\n").expect("Couldn't split input on double line break");
    let ranges: Vec<Range> = ranges.lines().map(|line| Range::new(line)).collect();
    let ingredients: Vec<i64> = ingredients.lines().map(|line| line.parse().expect("Couldn't parse ingredient")).collect();
    let mut fresh_count = 0;

    for i in ingredients {
        for range in &ranges {
            if range.contains(i) {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn solve2(input: &str) -> i64 {
    let (ranges, _) = input.split_once("\n\n").expect("Couldn't split input on double line break");
    let mut ranges: Vec<Range> = ranges.lines().map(|line| Range::new(line)).collect();
    loop {
        let mut merged = false;
        let mut counter = 0;
        'outer: while counter < ranges.len() {
            let current: Range = ranges.remove(0);
            for j in 0..ranges.len() {
                match current.merge(&ranges[j]) {
                    Some(r) => {
                        ranges.remove(j);
                        ranges.push(r);
                        merged = true;
                        break 'outer
                    },
                    None => continue
                }
            }
            ranges.push(current);
            counter += 1;
        }
        if !merged {
            break
        }
    }

    ranges.iter().map(|r| r.span()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = include_str!("../input-test.txt");
        assert_eq!(
            solve1(input),
            3
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

    #[test]
    fn merge_test() {
        let range1 = Range::new("5-10");
        let range2 = Range::new("1-2");
        let range3 = Range::new("11-12");
        let range4 = Range::new("4-5");
        let range5 = Range::new("4-6");
        let range6 = Range::new("4-11");
        let range7 = Range::new("9-11");
        let range8 = Range::new("10-11");

        assert_eq!(range1.merge(&range2), None);
        assert_eq!(range1.merge(&range3), None);
        assert_eq!(range1.merge(&range4), Some(Range::new("4-10")));
        assert_eq!(range1.merge(&range5), Some(Range::new("4-10")));
        assert_eq!(range1.merge(&range6), Some(Range::new("4-11")));
        assert_eq!(range1.merge(&range7), Some(Range::new("5-11")));
        assert_eq!(range1.merge(&range8), Some(Range::new("5-11")));
    }
}