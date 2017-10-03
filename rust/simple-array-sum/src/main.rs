use std::io;
fn main() {
    let mut r = String::new();
    io::stdin().read_line(&mut r).ok().expect(
        "failed to read line",
    );

    let mut total: u32 = 0;
    r = String::new();
    io::stdin().read_line(&mut r).ok().expect(
        "failed to read line",
    );
    let v: Vec<&str> = r.trim().split(' ').collect();
    for var in v {
        let v2: u32 = match var.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        total = total + v2;
    }
    println!("{}", total);
}
