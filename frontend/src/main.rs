use std::io::BufRead;

use backend::map_sum;

fn main() {
    let lines = std::io::stdin().lock().lines();

    let sum = map_sum(lines.map(|line| line.expect("Error reading input")));

    output_result(sum);
}

fn output_result(sum: usize) {
    // some nicer output needed :D
    println!("{}", sum);
}
