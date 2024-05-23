pub fn factorial(n: u32) -> u32 {
    //todo!()
    let mut sum = 1;
    for i in 1..(n+1) {
        sum *= i;
    }
    sum

}

fn main() {
    let value = factorial(5);
    println!("{}", value);
}