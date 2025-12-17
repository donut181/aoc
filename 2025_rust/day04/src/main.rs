fn main() {
    let input = include_str!("../input.txt");
    println!("Wynik 1: {}", solve1(input));
    println!("Wynik 2: {}", solve2(input));
}

struct Grid {
    data: Vec<Vec<u8>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Self { data: input.lines().map(|line| line.bytes().collect()).collect() }
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 {return b'.'}
        let x = x as usize;
        let y = y as usize;
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

    fn remove_roll(&mut self, x: usize, y: usize) {
        self.data[y][x] = b'x';
    }
}

fn solve1(input: &str) -> usize {
    let mut count = 0;
    let grid = Grid::new(input);
    for y in 0..grid.height() as i32 {
        for x in 0..grid.width() as i32 {
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

fn solve2(input: &str) -> i32 {
    let mut accessible_rolls:i32 = 0;
    let mut grid = Grid::new(input);
    loop {
        let mut coords_of_accessible_rolls: Vec<(usize, usize)> = Vec::new();
        for coord in coords(grid.width(), grid.height()) {
            let x = coord.0 as i32;
            let y = coord.1 as i32;
            let current_field = grid.get(x, y);
            if current_field == b'@' {
                if [(x-1, y), (x-1, y-1), (x-1, y+1), (x, y-1), (x, y+1), (x+1, y), (x+1, y-1), (x+1, y+1)]
                .iter().filter(|xy| grid.get(xy.0, xy.1) == b'@').count() < 4 {
                    coords_of_accessible_rolls.push(coord);
                }
            }
        }
        accessible_rolls += coords_of_accessible_rolls.len() as i32;
        if coords_of_accessible_rolls.len() == 0 {break}
        for coord in coords_of_accessible_rolls {
            grid.remove_roll(coord.0, coord.1);
        }
    }
    accessible_rolls
}

fn coords(range_x: usize, range_y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for y in 0..range_y {
        for x in 0..range_x {
            result.push((x,y));
        }
    }
    result
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

    #[test]
    fn test2() {
        let input = include_str!("../input-test.txt");
        assert_eq!(
            solve2(input),
            43
        )
    }
}
