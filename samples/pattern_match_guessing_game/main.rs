/// 猜数字游戏
fn main() {
    println!("Guess the number! 1-100");

    // 被猜的数字
    let secret_number: u64 = 41;

    loop {
        println!("Please input your guess.");

        // 标准输入（stdin）读取一行
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 字符串转为整型
        let guess: u64 = guess.trim().parse().unwrap();

        println!("You guessed: {}", guess);

        // 转为 Option 枚举类型
        let option_guess = Some(guess);

        // 比大小
        match option_guess {
            Some(x) if x < secret_number => {
                println!("Too small!");
            }
            Some(x) if x > secret_number => {
                println!("Too big!");
            }
            _ => {
                println!("You win!");
                break;
            }
        }
    }
}
