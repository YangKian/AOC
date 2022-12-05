use std::{collections::HashMap, collections::HashSet, str::FromStr};

pub fn solution_part_a(nums: &[u64], boards: &mut [Board]) {
    for &num in nums.iter() {
        for board in boards.iter_mut() {
            if board.mark_and_validate(num) {
                let res = board.total_sum * num;
                println!("res = {}", res);
                return;
            }
        }
    }
}

pub fn solution_part_b(nums: &[u64], boards: &mut [Board]) {
    let size = boards.len();
    println!("size = {}", size);
    let mut done: HashSet<usize> = HashSet::new();
    for &num in nums.iter() {
        for (idx, board) in boards.iter_mut().enumerate() {
            if !done.contains(&idx) && board.mark_and_validate(num) {
                done.insert(idx);
                println!("{} done.", idx);
                if done.len() == size {
                    let res = board.total_sum * num;
                    println!("res = {}", res);
                    return;
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Board {
    records: HashMap<u64, (usize, usize)>,
    total_sum: u64,
    rows_nums: Vec<usize>,
    cols_nums: Vec<usize>,
}

impl Board {
    fn new() -> Self {
        Self {
            records: HashMap::new(),
            total_sum: 0,
            rows_nums: Vec::new(),
            cols_nums: Vec::new(),
        }
    }

    fn mark_and_validate(&mut self, num: u64) -> bool {
        if let Some(&(row, col)) = self.records.get(&num) {
            self.total_sum -= num;
            self.rows_nums[row] -= 1;
            self.cols_nums[col] -= 1;
            return self.validate(row, col);
        }
        false
    }

    fn validate(&self, row: usize, col: usize) -> bool {
        self.rows_nums[row] == 0 || self.cols_nums[col] == 0
    }
}

impl FromStr for Board {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Board::new();
        s.lines().enumerate().for_each(|(row, line)| {
            let mut cnt = 0usize;
            for (col, c) in line.split_whitespace().enumerate() {
                let value = c.parse().unwrap();
                res.records.insert(value, (row, col));
                res.total_sum += value;
                cnt += 1;
            }
            res.rows_nums.push(cnt);
        });
        res.cols_nums = res.rows_nums.clone();
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, BorrowMut};
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = read_to_string("./src/day04/day4.txt")?;
        let mut iter = input.split("\n\n");
        let nums = iter
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect::<Vec<u64>>();
        let mut boards: Vec<Board> = iter.map(|s| s.parse().unwrap()).collect();
        solution_part_a(nums.borrow(), boards.borrow_mut());
        solution_part_b(nums.borrow(), boards.borrow_mut());
        Ok(())
    }
}
