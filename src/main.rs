use std::io;

fn fibonacci(num: u32) -> u32 {
    // if num == 0 {
    //     return 0;
    // }
    // let mut first_num = 0;
    // let mut second_num = 1;
    
    // for _ in 1..num {
    //     let temp = first_num + second_num;
    //     first_num = second_num;
    //     second_num = temp;
    // }
    
    // second_num
    let val: f32 = 5.0;
    let sqrt_val = val.sqrt();
    
    let fib = (1.0 + sqrt_val).powf(num as f32) / (sqrt_val * (2.0 as f32).powf(num as f32));

    fib.round() as u32
}

fn main() {
    let mut usr_input = String::new();

    println!("Enter the Fibonacci number index:");

    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to read line");

    let usr_input: u32 = usr_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let result = fibonacci(usr_input);
    println!("The Fibonacci number at position {} is: {}", usr_input, result);
}
