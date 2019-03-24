/*
 * An optimised search to find numbers with a large Multiplicative Persistence.
 * http://mathworld.wolfram.com/MultiplicativePersistence.html
 *
 * Inspired by this video: https://www.youtube.com/watch?v=Wim9WJeDTHQ
 */
use std::collections::BTreeSet;

fn main() {
    let mut queue = initial_queue();
    let mut seen = BTreeSet::new();

    let mut greatest = 0;
    let mut greatest_depth = 0;
    let mut last_shown = 0;

    while let Some((next, count)) = queue.pop() {
        if count > greatest_depth {
            greatest_depth = count;
            greatest = next;
            last_shown = count;
            println!("{:>4}: {:>30}", count, next);
        } else if count == greatest_depth && next < greatest {
            greatest = next;
            last_shown = count;
            println!("{:>4}: {:>30}", count, next);
        } else if count <= last_shown {
            last_shown = count;
            println!("{:>4}: {:>30}", count, next);
            println!("{:>4}: {:>30}", greatest_depth, greatest);
        }
        for factors in factor_combinations(next) {
            if factors != next && !seen.contains(&factors) {
                seen.insert(factors);
                queue.push((factors, count + 1))
            }
        }
    }
}

/// Prime the queue with numbers with small factors.
fn initial_queue() -> Vec<(u64, u32)> {
    (1..=9).into_iter().map(|i| (i, 0)).collect()
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
