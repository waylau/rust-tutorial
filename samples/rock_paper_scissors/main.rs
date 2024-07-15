/// 实现“剪刀石头布”游戏
fn main() {
    println!("Rock Sissors Paper! 1-3");

    println!("Player 1 guess.");

    // 标准输入（stdin）读取一行
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 字符串转为整型
    let player_guess1: u8 = guess.trim().parse().unwrap();

    println!("Player 1 guess.");

    // 标准输入（stdin）读取一行
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 字符串转为整型
    let player_guess2: u8 = guess.trim().parse().unwrap();

    println!("You guessed: {}", guess);

    // 比大小
    match player_guess1 {
        1 if player_guess2 == 2 => println!(
            "Player 1 win! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        1 if player_guess2 == 1 => println!(
            "Drew! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        1 if player_guess2 == 3 => println!(
            "Player 2 win! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        2 if player_guess2 == 3 => println!(
            "Player 1 win! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        2 if player_guess2 == 2 => println!(
            "Drew! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        2 if player_guess2 == 1 => println!(
            "Player 2 win! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        3 if player_guess2 == 3 => println!(
            "Player 1 win! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        3 if player_guess2 == 2 => println!(
            "Drew! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        3 if player_guess2 == 1 => println!(
            "Player 2 win! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
        _ => println!(
            "Illegal input! Player 1: {:?}, Player 2: {:?}",
            player_guess1, player_guess2
        ),
    }
}
