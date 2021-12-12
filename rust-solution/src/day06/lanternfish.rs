use std::str::FromStr;

pub fn solution_part_a(mut input: LifeCycle) {
    (0..80).for_each(|_| input.pass_one_day());
    println!("res = {}", input.count());
}

pub fn solution_part_b(mut input: LifeCycle) {
    (0..256).for_each(|_| input.pass_one_day());
    println!("res = {}", input.count());
}

// LifeCycle is a [0:8] days cycle, it contains a slice with 9 slot, each slot
// contains the num of fish in the index day
#[derive(Debug, Clone, Copy)]
pub struct LifeCycle {
    days: [usize; 9],
}

impl LifeCycle {
    fn new() -> Self {
        Self { days: [0; 9] }
    }

    fn pass_one_day(&mut self) {
        self.days.rotate_left(1);
        self.days[6] += self.days[8];
    }

    fn count(&self) -> usize {
        self.days.iter().sum()
    }
}

impl FromStr for LifeCycle {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .trim()
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .fold(LifeCycle::new(), |mut acc, x| {
                acc.days[x] += 1;
                acc
            });
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = read_to_string("./src/day06/day6.txt")?;
        let age: LifeCycle = input.parse().unwrap();
        solution_part_a(age);
        solution_part_b(age);
        Ok(())
    }
}
