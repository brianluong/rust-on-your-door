fn main() {
    let s = "salt air, and the rust on your door";
    let s2 = "i never wanted anything more";

    println!("count_chars: {}", count_chars(s));
    println!("count_chars_2: {}", count_chars_2(s.to_string()));
    println!("count_char_in_string: {}", count_char_in_string(s, 'l'));
    println!("join_strings: {}", join_strings(vec![s, s2], '!'));
    println!(
        "trim_whitespace: {}",
        trim_whitespace(format!("    {s}       ").as_str())
    );
}

fn count_chars(s: &str) -> i32 {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}

fn count_chars_2(s: String) -> i32 {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}

fn count_char_in_string(s: &str, c: char) -> i32 {
    let mut count = 0;
    for ch in s.chars() {
        if ch == c {
            count += 1;
        }
    }
    count
}

fn join_strings(v: Vec<&str>, delimeter: char) -> String {
    let mut final_str = String::new();
    for (i, str) in v.iter().enumerate() {
        if i != 0 {
            final_str.push_str(delimeter.to_string().as_str());
        }
        final_str.push_str(str);
    }

    final_str
}

fn trim_whitespace(s: &str) -> String {
    // Trim leading whitespace
    let mut first_pass_str = String::new();
    let mut reached_non_whitespace: bool = false;
    for c in s.chars() {
        if reached_non_whitespace {
            first_pass_str.push_str(c.to_string().as_str());
        } else if !c.is_whitespace() {
            reached_non_whitespace = true;
            first_pass_str.push_str(c.to_string().as_str());
        }
    }

    // Trim trailing whitespace
    let mut second_pass_str = String::new();
    reached_non_whitespace = false;
    for c in first_pass_str.chars().rev() {
        if reached_non_whitespace {
            second_pass_str = format!("{}{}", c.to_string(), second_pass_str);
        } else if !c.is_whitespace() {
            reached_non_whitespace = true;
            second_pass_str = format!("{}{}", c.to_string(), second_pass_str);
        }
    }

    second_pass_str
}
