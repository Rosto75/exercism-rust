pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();

    for c in num.to_string().chars() {
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit);
        }
    }

    let len = digits.len() as u32;
    let sum = digits.iter().map(|x| x.pow(len)).sum();

    num == sum
}
