use std::fs;
fn main() {
    // let contents = fs::read_to_string("input.txt").unwrap();
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut result: u32 = 0;
    let mut result2: u32 = 0;
    for line in contents.lines() {
        let str_len = get_actual_chars(line);
        println!("{}: {}", str_len, line);
        result = result + line.len() as u32 - str_len;
        result2 = result2 + get_encoded_chars(line) - line.len() as u32;
    }
    println!("{}", result);
    println!("{}", result2);
}

fn get_actual_chars(input: &str) -> u32 {
    let mut result: u32 = input.len() as u32;
    result -= 2; // because of extra surrounding quotes
    let mut prevCh: Option<char> = None;

    let mut i = 1;
    while i < input.len() - 1 {
        let ch = input.chars().nth(i).unwrap();
        if ch == '\\' {
            let next = input.chars().nth(i + 1).unwrap();
            if next == 'x' {
                result -= 3;
                i += 4;
            } else {
                result -= 1;
                i += 2;
            }
        } else {
            i += 1;
        }
    }
    result
}

fn get_encoded_chars(input: &str) -> u32 {
    let mut result: u32 = input.len() as u32 + 2;
    for ch in input.chars() {
        if ch == '\\' || ch == '"' {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::get_actual_chars;
    use std::fs;

    #[test]
    fn test_get_actual_chars() {
        let contents = fs::read_to_string("output.txt").unwrap();
        for line in contents.lines() {
            let splits: Vec<&str> = line.split(": ").collect();
            let expected: u32 = splits[0].trim().parse().unwrap();
            assert_eq!(
                expected,
                splits[1].len() as u32 - get_actual_chars(splits[1]),
                "String {}",
                line
            );
        }
    }
}
