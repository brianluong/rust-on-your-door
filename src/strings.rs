fn main() {
    let s = "hello";
    println!(
        "Counting characters in {s}, count: {}",
        count_chars("hello")
    );
}

fn count_chars(s: &str) -> i32 {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}
