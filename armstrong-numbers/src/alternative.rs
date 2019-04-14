// This solution is not mine, but I love it and wanted to share it with you.
// https://exercism.io/tracks/rust/exercises/armstrong-numbers/solutions/61da8a65024341b4b1b28d6b6a9a68f6
pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len() as u32;

    let sum = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(num_len))
        .sum();
    num == sum
}
