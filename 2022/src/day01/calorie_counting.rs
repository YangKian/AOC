pub fn solution_part_a(lines: &[Option<u64>]) {
    let groups = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max()
        .unwrap();
    println!("groups = {groups:?}");
}

pub fn solution_part_b(lines: &[Option<u64>]) {
    let mut groups = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .collect::<Vec<u64>>();
    groups.sort_unstable();
    let res = groups.into_iter().rev().take(3).sum::<u64>();
    println!("groups = {res:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let lines = include_str!("input.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .collect::<Vec<Option<u64>>>();
        solution_part_a(lines.as_slice());
        solution_part_b(lines.as_slice());
        Ok(())
    }
}
