/*
 * An optimised search to find numbers with a large Multiplicative Persistence.
 * http://mathworld.wolfram.com/MultiplicativePersistence.html
 *
 * Inspired by this video: https://www.youtube.com/watch?v=Wim9WJeDTHQ
 */
use std::collections::BinaryHeap;
use std::cmp::Ordering::{*, self};

#[derive(Ord, PartialEq, Eq)]
struct Pair {
    value: u64,
    count: u32,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.count > other.count {
            Some(Greater)
        } else if self.value < other.value {
            Some(Greater)
        } else if self.count == other.count && self.value == other.value {
            Some(Equal)
        } else {
            Some(Less)
        }
    }
}

fn main() {
    let mut queue = initial_queue();

    let mut greatest = 0;
    let mut greatest_depth = 0;
    let mut last_shown = 0;

    while let Some(Pair { value: next, count }) = queue.pop() {
        if count > greatest_depth {
            greatest_depth = count;
            greatest = next;
            last_shown = count;
            println!("{:>4}: {:>30}", count, next);
        } else if count == greatest_depth && next < greatest {
            greatest = next;
            last_shown = count;
            println!("{:>4}: {:>30}", count, next);
        } else if count != last_shown {
            last_shown = count;
            println!("{:>4}: {:>30} | {:>4}: {:>30}", greatest_depth, greatest, count, next);
        }
        for factors in factor_combinations(next) {
            if factors != next {
                queue.push(Pair { value: factors, count: count + 1 })
            }
        }
    }
}

/// Prime the queue with numbers with small factors.
fn initial_queue() -> BinaryHeap<Pair> {
    (1..=4).into_iter().map(|i| Pair { value: i, count: 0 }).collect()
}

/// Get all of the single digit factor combintions for a number.
fn factor_combinations(n: u64) -> impl Iterator<Item = u64> {
    fn next_factors(allow_one: bool, n: u64) -> impl Iterator<Item = (u64, u64)> {
        (if allow_one { 1 } else { 2 }..=9)
            .into_iter()
            .filter(move |f| n % f == 0)
            .map(move |f| (f, n / f))
    }

    let mut max_ones = 1;
    let mut limit = 10;
    while limit < n {
        limit *= limit;
        max_ones += 1;
    }

    fn allow_ones(max_ones: u64, factors: &Vec<u64>) -> bool {
        factors.iter().fold(0, |a, n| if *n == 1 { a + 1} else { a }) < max_ones
    }

    fn next_combinations(
        factors: Vec<u64>,
        remainder: u64,
        max_ones: u64,
    ) -> impl Iterator<Item = (Vec<u64>, u64)> {
        next_factors(allow_ones(max_ones, &factors), remainder)
            .map(move |(next, remainder)| {
                let mut new_factors = factors.clone();
                new_factors.push(next);
                (new_factors, remainder)
            })
            .flat_map(move |(factors, remainder)| {
                if remainder == 1 {
                    let next: Box<dyn Iterator<Item = (Vec<u64>, u64)>> =
                        Box::new(Some((factors, remainder)).into_iter());
                    next
                } else if remainder == 0 {
                    let next: Box<dyn Iterator<Item = (Vec<u64>, u64)>> =
                        Box::new(None.into_iter());
                    next
                } else {
                    let next: Box<dyn Iterator<Item = (Vec<u64>, u64)>> =
                        Box::new(next_combinations(factors, remainder, max_ones));
                    next
                }
            })
    }

    fn as_integer(xs: &Vec<u64>) -> u64 {
        let mut result = 0;
        for x in xs {
            result *= 10;
            result += x;
        }
        result
    }

    next_combinations(Vec::new(), n, max_ones).map(|(factors, _)| as_integer(&factors))
}
