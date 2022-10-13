use std::io;

#[derive(PartialEq, Eq)]
enum CoinFace {
    Heads,
    Tails,
}

fn main() {
    println!("Coin toss");
    println!("Welcome to the coin toss guessing game! The rules are simple, guess which side of the coin is showing when it lands - Heads or tails!");

    println!(" ");
    println!("Guess a side!");

    let coin_flip = match rand::random::<bool>() {
        true => CoinFace::Heads,
        false => CoinFace::Tails,
    };

    println!("Enter one of the following options:");
    println!(" ");
    println!("For Heads:");
    println!("  H, Head, Heads");
    println!(" ");
    println!("For Tails:");
    println!("  T, Tail, Tails");
    println!(" ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guessed_coin_face = match guess.trim() {
        "H" => Some(CoinFace::Heads),
        "Head" => Some(CoinFace::Heads),
        "Heads" => Some(CoinFace::Heads),
        "T" => Some(CoinFace::Tails),
        "Tail" => Some(CoinFace::Tails),
        "Tails" => Some(CoinFace::Tails),
        _ => None,
    };

    if guessed_coin_face.is_some() {
        if guessed_coin_face.unwrap() == coin_flip {
            println!("You guessed correct!");
        } else {
            println!("Wrong, try again.")
        }
    }
}
