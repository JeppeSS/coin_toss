use std::io;

#[derive(Debug, PartialEq, Eq)]
enum CoinFace {
    Heads,
    Tails,
}

fn main() {
    println!("Coin toss");
    println!("Welcome to the coin toss guessing game! The rules are simple, guess which side of the coin is showing when it lands - Heads or tails!");

    println!(" ");
    println!("*Cling* - The coin flips into the air!");
    println!(" ");

    let tossed_coin_face = match rand::random::<bool>() {
        true => CoinFace::Heads,
        false => CoinFace::Tails,
    };

    loop {
        println!("Guess a side!");
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
            let guessed_coin_face = guessed_coin_face.unwrap();
            println!(" ");
            println!("*Cling* - The coin lands on the ground");
            println!("Coin face showing: {:?}", guessed_coin_face);
            println!(" ");

            if guessed_coin_face == tossed_coin_face {
                println!("You guessed correct!");
                break;
            } else {
                println!("You lost.");
                break;
            }
        } else {
            println!("Invalid input, try again!");
        }
    }
}
