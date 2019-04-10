pub fn raindrops(n: u32) -> String {
    let mut output = String::new();
    let factors = vec![3, 5, 7];
    let sounds = vec!["Pling", "Plang", "Plong"];

    if n < factors[0] {
        return n.to_string();
    }

    for (i, factor) in factors.iter().enumerate() {
        if n % factor == 0 {
            output.push_str(sounds[i]);
        }
    }

    if output.is_empty() {
        output.push_str(&n.to_string());
    }

    output
}
