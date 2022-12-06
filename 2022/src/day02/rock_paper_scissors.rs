use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref STRATEGY1: HashMap<&'static str, i64> = HashMap::from([
        ("AX", 3 + 1),
        ("AY", 6 + 2),
        ("AZ", 3),
        ("BX", 1),
        ("BY", 3 + 2),
        ("BZ", 6 + 3),
        ("CX", 6 + 1),
        ("CY", 2),
        ("CZ", 3 + 3),
    ]);
    static ref STRATEGY2: HashMap<&'static str, i64> = HashMap::from([
        ("AX", 3),
        ("AY", 3 + 1),
        ("AZ", 6 + 2),
        ("BX", 1),
        ("BY", 3 + 2),
        ("BZ", 6 + 3),
        ("CX", 2),
        ("CY", 3 + 3),
        ("CZ", 6 + 1),
    ]);
}

pub fn solution_part_a(round: &[String]) {
    let res = round
        .iter()
        .map(|g| STRATEGY1.get(g.as_str()).unwrap())
        .sum::<i64>();
    println!("{res}")
}

pub fn solution_part_b(round: &[String]) {
    let res = round
        .iter()
        .map(|g| STRATEGY2.get(g.as_str()).unwrap())
        .sum::<i64>();
    println!("{res}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input.txt")
            .lines()
            .map(|line| line.replace(' ', ""))
            .collect::<Vec<String>>();
        println!("{input:?}");
        solution_part_a(input.as_slice());
        solution_part_b(input.as_slice());
    }
}
