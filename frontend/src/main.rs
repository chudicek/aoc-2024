use backend::line_sum_active;

fn main() {
    println!("Hello, world!");

    let previous_line = "abc";
    let analyzed_line = "def";
    let next_line = "ghi";

    let _ = line_sum_active(previous_line, analyzed_line, next_line);
}
