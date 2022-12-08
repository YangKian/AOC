use std::collections::HashSet;

pub fn solution_part_a(inputs: &[&str]) {
    let mut sum = 0usize;
    for &input in inputs.iter() {
        let mut seen = HashSet::with_capacity(52);
        let mut chars = input.bytes();
        let mut mid = (input.len() / 2) as i32;
        while mid > 0 {
            seen.insert(chars.next().unwrap());
            mid -= 1;
        }
        for c in chars {
            if seen.contains(&c) {
                match c {
                    b'a'..=b'z' => sum += (c - b'a') as usize + 1,
                    b'A'..=b'Z' => sum += (c - b'A') as usize + 27,
                    _ => unreachable!(),
                }
                break;
            }
        }
    }
    println!("{sum}")
}

pub fn solution_part_b(inputs: &[&str]) {
    let mut sum = 0usize;
    let mut seen = vec![0; 53];
    for (idx, &input) in inputs.iter().enumerate() {
        for c in input.bytes() {
            let priority = match c {
                b'a'..=b'z' => ((c - b'a') + 1) as usize,
                b'A'..=b'Z' => ((c - b'A') + 27) as usize,
                _ => unreachable!(),
            };
            seen[priority] |= 1 << (idx % 3);

            if seen[priority] == 7 {
                sum += priority;
                seen = vec![0; 53];
                break;
            }
        }
    }
    println!("{sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input.txt").lines().collect::<Vec<&str>>();
        solution_part_a(input.as_slice());
        solution_part_b(input.as_slice());
    }
}
