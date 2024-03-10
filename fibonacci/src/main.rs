use std::io;

fn fibonacci(n: u32) -> i128 {
    let mut fib_0 = 0;
    let mut fib_1 = 1;
    let mut result = 0;

    for x in 0..n {
        if x == 0 {
            result = fib_1
        } else {
            result = fib_1 + fib_0;
            fib_0 = fib_1;
            fib_1 = result;
        }
    }

    result
}

fn main() {
    'fib: loop {
        println!("Write your number!");

        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => break 'fib,
        };

        println!("Result is {}", fibonacci(user_input))
    }
}
