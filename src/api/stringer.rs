pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn inspect(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        (input.len() as i32, "char".to_string())
    } else {
        (inspect_numbers(input), "digit".to_string())
    }
}

fn inspect_numbers(input: &str) -> i32 {
    input.chars().filter(|ch| ch.is_ascii_digit()).count() as i32
}
