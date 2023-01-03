use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use std::cmp::max;
use std::fmt;

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn parse_crate(c: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(c)
}

fn parse_hole(c: &str) -> IResult<&str, ()> {
    // consume three space, and return nothing
    map(tag("   "), drop)(c)
}

fn parse_crate_or_hole(c: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(c)
}

// The patten of a line is: first_item + [space + remain_items],
// first_item can be a crate or hole.
// So we need to eat first_item first, then eat [space + remain_items] multi-times
fn parse_crate_line(c: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut remain, res) = parse_crate_or_hole(c)?;
    let mut v = vec![res];

    loop {
        let (next, res) = opt(preceded(tag(" "), parse_crate_or_hole))(remain)?;
        // every time we get a crate or hole, we both get a Some(c).
        // return None only when we meet error or we have consume all characters.
        match res {
            Some(c) => v.push(c),
            None => break,
        }
        remain = next;
    }
    Ok((remain, v))
}

// convert a row vec to a column vec, also ignore None result
fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

// convert from 1-indexed to 0-indexed
fn parse_pile_number(c: &str) -> IResult<&str, usize> {
    map(parse_number, |c| c - 1)(c)
}

struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_instruction(c: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(c)
}

fn solution_part_a(p: &Piles) {
    let res = p.0.iter().map(|pile| pile.last().unwrap()).join("");
    // let res = res.join("");
    println!("res = {res:?}");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn test() {
        let mut lines = include_str!("input.txt").lines();

        let crate_lines: Vec<_> = (&mut lines)
            .map_while(|line| {
                all_consuming(parse_crate_line)(line)
                    .finish()
                    .ok()
                    .map(|(_, line)| line)
            })
            .collect();

        let mut piles = Piles(transpose_rev(crate_lines));
        println!("{piles:?}");

        // we've consumed the "numbers line" but not the separating line
        assert!(lines.next().unwrap().is_empty());

        for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
            println!("{ins:?}");
            piles.apply(ins);
            println!("{piles:?}");
        }

        solution_part_a(piles.borrow())
    }
}
