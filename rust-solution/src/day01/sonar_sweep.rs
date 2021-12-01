pub fn solution_part_a(input: &[i32]) {
    let mut previous = input[0];
    let res = input.iter().fold(0, |mut acc, &x| {
        if x.gt(&previous) {
            acc += 1;
        }
        previous = x;
        acc
    });
    println!("res = {:?}", res);
}

pub fn solution_part_b(input: &[i32]) {
    let mut previous: i32 = input[..3].iter().sum();
    println!("previous = {}", previous);
    let res = input.windows(3).fold(0, |mut acc, sub| {
        let window_sum = sub.iter().sum::<i32>();
        if window_sum.gt(&previous) {
            acc += 1;
        }
        previous = window_sum;
        acc
    });
    println!("res = {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input: Vec<i32> = std::fs::read_to_string("./src/day01/day1.txt")?
            .lines()
            .flat_map(|s| s.parse::<i32>())
            .collect();
        solution_part_a(&input);
        solution_part_b(&input);
        Ok(())
    }
}
