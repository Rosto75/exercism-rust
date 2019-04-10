pub fn verse(n: i32) -> String {
    if n > 99 || n < 0 {
        return String::new();
    }

    let (mut count1, mut count2, mut count3, mut bottles1, mut bottles2, wall, mut action) = (
        n.to_string(),
        n.to_string(),
        (n - 1).to_string(),
        String::from("bottles of beer"),
        String::from("bottles of beer"),
        String::from("on the wall"),
        String::from("Take one down and pass it around"),
    );

    if n == 2 {
        bottles2 = String::from("bottle of beer");
    } else if n == 1 {
        count3 = String::from("no more");
        bottles1 = String::from("bottle of beer");
        bottles2 = String::from("bottles of beer");
        action = String::from("Take it down and pass it around");
    } else if n == 0 {
        count1 = String::from("No more");
        count2 = String::from("no more");
        count3 = String::from("99");
        action = String::from("Go to the store and buy some more");
    };

    format!(
        "{0} {3} {5}, {1} {3}.\n{6}, {2} {4} {5}.\n",
        count1, count2, count3, bottles1, bottles2, wall, action
    )
}

pub fn sing(start: i32, end: i32) -> String {
    let mut result = String::new();

    for i in (end..=start).rev() {
        result.push_str(&verse(i));
        if i > end {
            result.push_str("\n");
        }
    }

    result
}
