use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("[Game]: Guess the number!");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    // 获取键盘输入
    println!("Please input your guess!");
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    // compare secret and guess
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win"),
    }
}
