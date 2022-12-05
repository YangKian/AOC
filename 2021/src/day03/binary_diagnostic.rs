pub fn solution_part_a(input: &[String]) {
    let cols = input[0].len();
    let col_mp = input.iter().fold(vec![0; cols], |mut acc, line| {
        for (idx, c) in line.chars().enumerate() {
            match c {
                '1' => acc[idx] += 1,
                '0' => acc[idx] -= 1,
                _ => panic!("wrong input"),
            }
        }
        acc
    });
    let gamma_rate = col_mp.iter().fold(
        0usize,
        |acc, &x| if x > 0 { acc << 1 | 1 } else { acc << 1 },
    );
    let epsilon_rate = col_mp.iter().fold(
        0usize,
        |acc, &x| if x > 0 { acc << 1 } else { acc << 1 | 1 },
    );
    println!("res {}", gamma_rate * epsilon_rate);
}

fn get_result<F>(input: &[usize], offset: usize, pred: F) -> usize
where
    F: Fn(usize, usize) -> bool,
{
    let mut records: Vec<usize> = input.to_vec();
    let mut mask = offset - 1;
    loop {
        let size = records.len();
        let pivot = records
            .iter_mut()
            .partition_in_place(|&x| ((x >> mask) & 1) == 1);
        if pred(pivot, size - pivot) {
            records = records[..pivot].into();
        } else {
            records = records[pivot..].into();
        }
        if records.len() == 1 {
            break;
        }
        mask -= 1;
    }

    records[0]
}

pub fn solution_part_b(input: &[String]) {
    let cols = input[0].len();
    let nums = input
        .iter()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();
    let find_oxy = |start1: usize, start0: usize| start1 >= start0;
    let find_co2 = |start1: usize, start0: usize| start1 < start0;

    let oxy_rate = get_result(&nums, cols, find_oxy);
    let co2_rate = get_result(&nums, cols, find_co2);
    println!("res = {}", oxy_rate * co2_rate);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = read_to_string("./src/day03/day3.txt")?
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        solution_part_a(input.as_ref());
        solution_part_b(input.as_ref());
        Ok(())
    }
}
