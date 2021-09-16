///
/// Generate the nth fibonacci number
/// 

fn main() {
    for num in 1..100 {
        println!("The fibonacci number when n is {:?} is {:?}", num, fibonacci(num));
    }
}


fn fibonacci(n: u32) -> u128 {
    if n == 0  {
        return 0;
    }

    if n == 1  {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}