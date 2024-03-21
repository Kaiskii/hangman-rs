use std::env;
use std::io;
use reqwest::Client;
use reqwest::Result;
use dotenvy::dotenv;
use serde::{Deserialize};

#[derive(Deserialize)]
struct WordAPI {
    word: String,
}

async fn get_random_word() -> Result<String> {
    let request_url = env::var("WORD_API").expect("WORD_API not found") + "/word";
    let response = Client::new()
        .get(request_url)
        .send()
        .await?
        .json::<WordAPI>()
        .await?;

    Ok(response.word)
}

fn game_loop(word_ref: &str) {
    let mut buffer = String::from("");
    let mut correct = String::from("");
    let mut history = String::from("");

    loop {
        let mut blank = String::from("");

        // Logic to process words
        for c in word_ref.chars() {
            if buffer.len() > 0 {
                let answer = buffer.chars().next().unwrap();
                if c == answer
                {
                    blank += &answer.to_string();
                    if !correct.contains(&answer.to_string()) {
                        correct += &answer.to_string();
                    }
                } else if correct.contains(c) {
                    blank += &c.to_string();
                } else {
                    blank += "|";
                }
            } else {
                blank += "|";
            }

            // Win Condition
            if blank == word_ref {
                // CLS
                print!("\x1B[2J\x1B[1;1H");

                // Victory
                println!("It was {}!", word_ref);
                println!("You got it in {} tries.", history.len());
                return;
            }
        }

        // Status Update
        println!("{}", blank);
        println!("{} Characters in Word", word_ref.len());
        println!("Guessed {}", history);

        // Refresh buffer
        buffer = String::from("");

        println!("What is Character would you like to guess?");
        while buffer.len() != 1 {
            io::stdin().read_line(&mut buffer).expect("failed to receive input");
            buffer = String::from(buffer.trim());

            if buffer.len() == 1 {
                history += &buffer;
            } else {
                buffer = String::from("");
                println!("Please input only 1 Char");
            }
        }
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    // Setup dotenv
    dotenv().expect("dotenv initialization failed");

    // Setup blanks and Get initial word
    let word = get_random_word().await?;

    game_loop(&word);

    Ok(())
}
