use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess!");
    let mut guess = String::new();
    // 获取键盘输入
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed : {}", guess);
}