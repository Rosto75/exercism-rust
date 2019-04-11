use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut map = HashMap::new();

    for num in 1..limit {
        for factor in factors {
            if *factor != 0 && num % factor == 0 {
                map.entry(num).or_insert(1);
            }
        }
    }

    map.iter().map(|(k, _)| k).sum()
}

// This version is modified version of someone elses solution:
// https://exercism.io/tracks/rust/exercises/sum-of-multiples/solutions/71f7a08fcac34202bec795163eee2697
// I added a zero check, so it could pass one of the tests.
pub fn sum_of_multiples2(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| factors.iter().any(|f| *f != 0 && i % f == 0))
        .sum()
}
