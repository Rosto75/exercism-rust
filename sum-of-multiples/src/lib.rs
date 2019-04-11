use std::collections::HashMap;

pub fn sum_of_multiples2(limit: u32, factors: &[u32]) -> u32 {
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

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| factors.iter().any(|f| *f != 0 && i % f == 0))
        .sum()
}
