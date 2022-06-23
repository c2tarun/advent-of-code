fn main() {
    // println!("{}", look_n_say("111221"));
    let mut next = String::from("1113222113");
    // let mut next = String::from("1");
    for i in 0..50 {
        next = look_n_say(next);
        // println!("{}: {}", i, next);
    }
    println!("{}", next.len());
}

fn look_n_say(num: String) -> String {
    let mut output: String = String::new();
    let mut running: Option<char> = None;
    let mut count = 0;
    for ch in num.chars() {
        match running {
            Some(v) => {
                if v == ch {
                    count += 1;
                } else {
                    output.push(char::from_digit(count, 10).unwrap());
                    output.push(running.unwrap());
                    running = Some(ch);
                    count = 1;
                }
            }
            None => {
                running = Some(ch);
                count = 1;
            }
        }
    }
    output.push(char::from_digit(count, 10).unwrap());
    output.push(running.unwrap());

    output
}
