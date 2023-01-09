fn find_longest_substring(s: &str, target: usize) -> usize {
    let size = s.len();

    let (mut slow, mut fast) = (0, 0);
    let mut mp: Vec<usize> = vec![0; 128];
    let input = s.as_bytes();
    let mut count = 0;
    while fast < size {
        let c = input[fast] as usize;
        while mp[c] == 1 {
            mp[input[slow] as usize] -= 1;
            count -= 1;
            slow += 1;
        }

        if count == target {
            return fast;
        }

        mp[c] += 1;
        fast += 1;
        count += 1;
    }
    0
}

fn solution_for_part_a(s: &str) {
    let res = find_longest_substring(s, 4);
    println!("{res}")
}

fn solution_for_part_b(s: &str) {
    let res = find_longest_substring(s, 14);
    println!("{res}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let content = include_str!("input.txt").to_string();
        solution_for_part_a(content.as_str());
        solution_for_part_b(content.as_str());
    }
}
