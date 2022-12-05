use std::str::FromStr;

pub fn solution_part_a(input: &[Vec<Position>], max_x: usize, max_y: usize) {
    let graph = input
        .iter()
        .fold(vec![vec![0i64; max_x + 1]; max_y + 1], |mut acc, p| {
            if p[0].x == p[1].x {
                let x = p[0].x;
                let from = p[0].y.min(p[1].y);
                let to = p[0].y.max(p[1].y);
                for y in from..=to {
                    acc[x as usize][y as usize] += 1;
                }
            } else if p[0].y == p[1].y {
                let y = p[0].y;
                let from = p[0].x.min(p[1].x);
                let to = p[0].x.max(p[1].x);
                for x in from..=to {
                    acc[x as usize][y as usize] += 1;
                }
            }
            acc
        });

    let total = graph.iter().fold(0, |acc, row| {
        let count = row.iter().filter(|&v| *v >= 2).count();
        acc + count
    });
    println!("res = {}", total);
}

pub fn solution_part_b(input: &[Vec<Position>], max_x: usize, max_y: usize) {
    let graph = input
        .iter()
        .fold(vec![vec![0i64; max_x + 1]; max_y + 1], |mut acc, p| {
            if p[0].x == p[1].x {
                let x = p[0].x;
                let from = p[0].y.min(p[1].y);
                let to = p[0].y.max(p[1].y);
                for y in from..=to {
                    acc[x as usize][y as usize] += 1;
                }
            } else if p[0].y == p[1].y {
                let y = p[0].y;
                let from = p[0].x.min(p[1].x);
                let to = p[0].x.max(p[1].x);
                for x in from..=to {
                    acc[x as usize][y as usize] += 1;
                }
            } else {
                let a = (p[1].y as i64 - p[0].y as i64) / (p[1].x as i64 - p[0].x as i64);
                if a.abs() == 1 {
                    let end = p[0].x.max(p[1].x);
                    let start;
                    let mut y;
                    if p[0].x < p[1].x {
                        start = p[0].x;
                        y = p[0].y;
                        acc[start as usize][y as usize] += 1;
                    } else {
                        start = p[1].x;
                        y = p[1].y;
                        acc[start as usize][y as usize] += 1;
                    }

                    for x in (start + 1)..=end {
                        y += a;
                        acc[x as usize][y as usize] += 1;
                    }
                }
            }
            acc
        });

    let total = graph.iter().fold(0, |acc, row| {
        let count = row.iter().filter(|&v| *v >= 2).count();
        acc + count
    });
    println!("res = {}", total);
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl FromStr for Position {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        Ok(Self {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let (mut max_x, mut max_y) = (0, 0);
        let input = read_to_string("./src/day05/day5.txt")?
            .lines()
            .map(|line| {
                let positions = line
                    .split(" -> ")
                    .map(|p| p.parse().unwrap())
                    .collect::<Vec<Position>>();
                max_x = max_x.max(positions[0].x).max(positions[1].x);
                max_y = max_y.max(positions[0].y).max(positions[1].y);
                positions
            })
            .collect::<Vec<_>>();
        solution_part_a(input.as_ref(), max_x as usize, max_y as usize);
        solution_part_b(input.as_ref(), max_x as usize, max_y as usize);
        Ok(())
    }
}
