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

    let mut is_running = true;
    while is_running {
        println!("Guess a side!");
        println!("Enter one of the following options:");
        println!(" ");
        println!("For Heads:");
        println!("  H, Head, Heads");
        println!(" ");
        println!("For Tails:");
        println!("  T, Tail, Tails");
        println!(" ");

        let mut input = String::new();
        match io::stdin()
            .read_line(&mut input) {
                Err(error) => {
                    println!("Could not read input: {}", error);
                    continue;
                }
                _ =>(),
            }

        let guessed_coin_face = match input.trim() {
            "H" => Some(CoinFace::Heads),
            "Head" => Some(CoinFace::Heads),
            "Heads" => Some(CoinFace::Heads),
            "T" => Some(CoinFace::Tails),
            "Tail" => Some(CoinFace::Tails),
            "Tails" => Some(CoinFace::Tails),
            _ => None,
        };


        match guessed_coin_face {
            Some(coin_face) => {
                println!(" ");
                println!("*Cling* - The coin lands on the ground");
                println!("Coin face showing: {:?}", coin_face);
                println!(" ");
                if coin_face == tossed_coin_face {
                    println!("You guessed correct!");
                    is_running = false;
                } else {
                    println!("You lost.");
                    is_running = false; 
                }
            },
            None =>  println!("Invalid input, try again!"),
        }
    }
}
