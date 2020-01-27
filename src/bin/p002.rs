#[macro_use(problem)]
extern crate common;
extern crate iterators;

use iterators::fib::Fib;

fn compute(bound: u32) -> u32 {
    Fib::<u32>::new()
        .take_while(|&n| n < bound)
        .filter(|&n| n % 2 == 0)
        .sum()
}

fn solve() -> String {
    compute(4_000_000u32).to_string()
}

problem!("4613732", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sum_first_ten_terms() {
        let terms = [2, 8, 34];
        let sum: u32 = terms.iter().sum();
        assert_eq!(sum, super::compute(100));
    }
}
