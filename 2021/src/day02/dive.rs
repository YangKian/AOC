#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Forward,
    Down,
    Up,
}

#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub dir: Dir,
    pub step: i32,
}

pub fn solve_part_a(input: &[Command]) {
    let res = input.iter().fold((0, 0), |acc, &c| match c.dir {
        Dir::Forward => (acc.0 + c.step, acc.1),
        Dir::Down => (acc.0, acc.1 + c.step),
        Dir::Up => (acc.0, acc.1 - c.step),
    });
    println!("{:?}", res);
    println!("res = {}", res.0 * res.1);
}

pub fn solve_part_b(input: &[Command]) {
    let mut aim = 0;
    let res = input.iter().fold((0, 0), |acc, &c| match c.dir {
        Dir::Forward => (acc.0 + c.step, acc.1 + aim * c.step),
        Dir::Down => {
            aim += c.step;
            acc
        }
        Dir::Up => {
            aim -= c.step;
            acc
        }
    });
    println!("{:?}", res);
    println!("res = {}", res.0 * res.1);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = read_to_string("./src/day02/day2.txt")?
            .lines()
            .map(|l| {
                let mut token = l.split_whitespace();
                Command {
                    dir: match token.next() {
                        Some(tok) => match tok {
                            "forward" => Dir::Forward,
                            "up" => Dir::Up,
                            "down" => Dir::Down,
                            _ => panic!("unknown commond {}", tok),
                        },
                        None => panic!("get an empty command in line {}", l),
                    },
                    step: match token.next() {
                        Some(tok) => tok.parse::<i32>().unwrap(),
                        None => panic!("step should not be empty in line {}", l),
                    },
                }
            })
            .collect::<Vec<Command>>();
        solve_part_a(&input);
        solve_part_b(&input);
        Ok(())
    }
}
