use std::io;

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let mut r = String::new();
    io::stdin().read_line(&mut r).ok().expect(
        "failed to read line",
    );
    let numbers: Vec<i64> = get_input()
        .split_whitespace()
        .map(|x| x.parse::<i64>().expect("parse error"))
        .collect::<Vec<i64>>();

    let mut total: i64 = 0;
    for n in numbers {
        total = total + n;
    }
    println!("{}", total)
}
