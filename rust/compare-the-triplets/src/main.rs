use std::io;
fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
fn main() {
    let numbersA: Vec<i32> = get_input().split_whitespace()
    .map(|x| x.parse::<i32>().expect("parse error"))
    .collect::<Vec<i32>>();
    let numbersB: Vec<i32> = get_input().split_whitespace()
    .map(|x| x.parse::<i32>().expect("parse error"))
    .collect::<Vec<i32>>();
   let mut totalA = 0;
   let mut totalB = 0;
   for index in 0..3{
       if (numbersA[index] > numbersB[index]) {
        totalA = totalA + 1;
       }
       if (numbersA[index] < numbersB[index]) {
        totalB = totalB + 1;
       }
   }
   println!("{} {}",totalA,totalB)

}
