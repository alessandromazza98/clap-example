pub fn reverse(input: &String) -> String {
    input.chars().rev().collect()
}

pub fn inspect(input: &String, digits: bool) -> (i32, String) {
    if !digits {
        return (input.len() as i32, "char".to_string());
    } else {
        (inspect_numbers(input), "digit".to_string())
    }
}

fn inspect_numbers(input: &String) -> i32 {
    input.chars().filter(|ch| ch.is_digit(10)).count() as i32
}