fn main() {
    println!("Hello, world!");
}

struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Self { data: input.lines().map(|line| line.bytes().collect()).collect() }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        match self.data.get(y).and_then(|row| row.get(x).copied()) {
            Some(value) => value,
            None => b'.',
        }
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }
}

fn solve1(input: &str) -> usize {
    let mut count = 0;
    let grid = Grid::new(input);
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let value = grid.get(x, y);
            if value == b'@' {
                if [(x-1, y), (x-1, y-1), (x-1, y+1), (x, y-1), (x, y+1), (x+1, y), (x+1, y-1), (x+1, y+1)]
                .iter().filter(|xy| grid.get(xy.0, xy.1) == b'@').count() < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = include_str!("../input-test.txt");
        assert_eq!(
            solve1(input),
            13
        );
    }
}
