pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors: Vec<u64> = Vec::new();

    let mut factor = 2;

    while n > 1 {
        while n % factor == 0 {
            n /= factor;
            factors.push(factor);
        }
        factor += 1;
    }

    factors
}
