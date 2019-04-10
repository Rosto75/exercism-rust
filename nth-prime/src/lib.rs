pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2];
    let mut len = 1;
    let mut flag: bool;

    for i in 3..u32::max_value() {
        flag = true;

        for p in &primes[0..len] {
            if i % p == 0 {
                flag = false;
                break;
            }
        }
        if flag == true {
            &mut primes.push(i);
            len += 1;

            if len > n as usize {
                break;
            }
        }
    }

    primes[n as usize]
}
