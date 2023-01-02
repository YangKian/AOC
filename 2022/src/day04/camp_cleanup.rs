use anyhow::format_err;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct EndPoint(i32, i32);

impl FromStr for EndPoint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once('-')
            .and_then(|(l, r)| {
                let start = l.parse::<i32>();
                let end = r.parse::<i32>();
                match (start, end) {
                    (Ok(s), Ok(e)) => Some(EndPoint(s, e)),
                    _ => None,
                }
            })
            .ok_or(format_err!("parse {s} error"))
    }
}

impl EndPoint {
    fn full_contains(self, other: &EndPoint) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn partial_contains(self, other: &EndPoint) -> bool {
        self.0 <= other.0 && self.1 >= other.0
    }
}

fn solution_part_a(input: &[(EndPoint, EndPoint)]) {
    let res = input.iter().fold(0, |acc, (l, r)| {
        if l.full_contains(r) || r.full_contains(l) {
            return acc + 1;
        }
        acc
    });
    println!("{res}")
}

fn solution_part_b(input: &[(EndPoint, EndPoint)]) {
    let res = input.iter().fold(0, |acc, (l, r)| {
        if l.partial_contains(r) || r.partial_contains(l) {
            return acc + 1;
        }
        acc
    });
    println!("{res}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input.txt")
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let start = parts
                    .next()
                    .map(|s| s.parse::<EndPoint>().unwrap())
                    .unwrap();
                let end = parts
                    .next()
                    .map(|s| s.parse::<EndPoint>().unwrap())
                    .unwrap();
                (start, end)
            })
            .collect::<Vec<_>>();
        solution_part_a(input.as_slice());
        solution_part_b(input.as_slice());
    }
}
