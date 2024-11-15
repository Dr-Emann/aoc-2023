use aoc_runner_derive::aoc;

pub struct Grid<'a> {
    stride: usize,
    data: &'a [u8],
}

#[derive(Debug)]
enum NumberVertical {
    None,
    One(u32),
    Two(u32, u32),
}

impl Grid<'_> {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[xy_to_idx(x, y, self.stride)]
    }

    fn number_left_of(&self, x: usize, y: usize) -> Option<u32> {
        let last_digit = self.get(x.checked_sub(1)?, y);
        if !last_digit.is_ascii_digit() {
            return None;
        }
        let mut res = u32::from(last_digit - b'0');
        let mut base = 1;
        for x in (0..x - 1).rev() {
            let ch = self.get(x, y);
            if !ch.is_ascii_digit() {
                break;
            }
            base *= 10;
            res += u32::from(ch - b'0') * base;
        }
        Some(res)
    }

    fn number_right_of(&self, x: usize, y: usize) -> Option<u32> {
        let first_x = x + 1;
        if first_x > self.stride {
            return None;
        }
        let first_digit = self.get(first_x, y);
        if !first_digit.is_ascii_digit() {
            return None;
        }
        let mut res = u32::from(first_digit - b'0');
        for x in first_x + 1..self.stride {
            let ch = self.get(x, y);
            if !ch.is_ascii_digit() {
                break;
            }
            res = res * 10 + u32::from(ch - b'0');
        }
        Some(res)
    }

    fn number_and_bounds(&self, x: usize, y: usize) -> Option<(u32, (usize, usize))> {
        let ch = self.get(x, y);
        if !ch.is_ascii_digit() {
            return None;
        }
        let mut base = 1;
        let mut res = u32::from(ch - b'0');
        let right = 'right: {
            for x in x + 1..self.stride {
                let ch = self.get(x, y);
                if !ch.is_ascii_digit() {
                    break 'right x;
                }
                res = res * 10 + u32::from(ch - b'0');
                base *= 10;
            }
            self.stride
        };
        let left = 'left: {
            for x in (0..x).rev() {
                let ch = self.get(x, y);
                if !ch.is_ascii_digit() {
                    break 'left x;
                }
                base *= 10;
                res += u32::from(ch - b'0') * base;
            }
            0
        };

        Some((res, (left, right)))
    }

    fn number_in_3(&self, x: usize, y: usize) -> NumberVertical {
        let number_up_left = if x == 0 {
            None
        } else {
            self.number_and_bounds(x - 1, y)
        };
        let number_up_right = if number_up_left.map_or(true, |(_, (_, right))| right < x) {
            self.number_and_bounds(x + 1, y)
        } else {
            None
        };

        match (number_up_left, number_up_right) {
            (None, None) => {
                let ch = self.get(x, y);
                if !ch.is_ascii_digit() {
                    NumberVertical::None
                } else {
                    NumberVertical::One(u32::from(ch - b'0'))
                }
            }
            (Some((n, _)), None) | (None, Some((n, _))) => NumberVertical::One(n),
            (Some((n1, _)), Some((n2, _))) => NumberVertical::Two(n1, n2),
        }
    }

    fn each_number<F>(&self, x: usize, y: usize, mut f: F)
    where
        F: FnMut(u32, usize, usize),
    {
        if let Some(left) = self.number_left_of(x, y) {
            f(left, x - 1 - left.ilog10() as usize, y);
        }
        if let Some(right) = self.number_right_of(x, y) {
            f(right, x + 1, y);
        }
        if y > 0 {
            match self.number_in_3(x, y - 1) {
                NumberVertical::None => {}
                NumberVertical::One(n) => f(n),
                NumberVertical::Two(n1, n2) => {
                    f(n1);
                    f(n2);
                }
            }
        }
        if y + 1 <= self.data.len() / self.stride {
            match self.number_in_3(x, y + 1) {
                NumberVertical::None => {}
                NumberVertical::One(n) => f(n),
                NumberVertical::Two(n1, n2) => {
                    f(n1);
                    f(n2);
                }
            }
        }
    }
}

fn xy_to_idx(x: usize, y: usize, stride: usize) -> usize {
    y * stride + x
}

fn idx_to_xy(idx: usize, stride: usize) -> (usize, usize) {
    (idx % stride, idx / stride)
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let line_len = input.lines().next().unwrap().len() + 1;
    let grid = Grid {
        stride: line_len,
        data: input.as_bytes(),
    };

    let mut res = 0;
    for (i, ch) in input.bytes().enumerate() {
        if ch == b'\n' || ch == b'.' {
            continue;
        }
        if ch.is_ascii_digit() {
            continue;
        }
        let (x, y) = idx_to_xy(i, line_len);
        grid.each_number(x, y, |n| {
            dbg!(grid.get(x, y) as char, n);
            res += n;
        });
    }
    res
}

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("<EXAMPLE>"), "<RESULT>");
    }
}
