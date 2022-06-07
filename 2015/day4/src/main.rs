use std::collections::HashSet;

use md5;

fn main() {
    day5();
}

fn day5() {
    let mut nice_strings = 0;
    let content = std::fs::read_to_string("day5input.txt").unwrap();
    for line in content.lines() {
        if _pair_appearing_twice(line) && _three_letter_palindrome(line) {
            nice_strings += 1;
        }
    }
    println!("Nice strings: {}", nice_strings);
}

fn _pair_appearing_twice(name: &str) -> bool {
    for i in 0..name.len() - 1 {
        let pair: &str = &name[i..i + 2];
        match name.rfind(pair) {
            Some(v) => {
                if v as i32 - i as i32 >= 2 {
                    return true;
                }
            }
            None => (),
        }
    }
    return false;
}

fn _three_letter_palindrome(name: &str) -> bool {
    for i in 0..name.len() - 2 {
        let ch_left = name.chars().nth(i).unwrap();
        let ch_right = name.chars().nth(i + 2).unwrap();

        if ch_left == ch_right {
            return true;
        }
    }
    return false;
}

fn _three_vowels(name: &str, vowels: &HashSet<char>) -> bool {
    let mut count = 0;
    for ch in name.chars() {
        if vowels.contains(&ch) {
            count += 1;
        }
        if count == 3 {
            return true;
        }
    }
    return false;
}

fn _twice_in_row(name: &str) -> bool {
    let mut prev: char = ' ';
    for ch in name.chars() {
        if prev == ch {
            return true;
        }
        prev = ch;
    }
    return false;
}

fn _no_bad_strings(name: &str, bad_strings: &Vec<&str>) -> bool {
    for bad_string in bad_strings {
        match name.find(bad_string) {
            Some(_) => return false,
            None => (),
        }
    }
    return true;
}

#[allow(dead_code)]
fn day4() {
    let secret_key = "iwrupvqb";
    let mut counter = 1;
    loop {
        let digest = md5::compute(format!("{}{}", secret_key, counter).as_bytes());
        if format!("{:x}", digest).starts_with("000000") {
            println!("The number: {}", counter);
            break;
        }
        counter += 1;
    }
}
