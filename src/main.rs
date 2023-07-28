// use std::thread;
// use std::time::Duration;
use std::io::{Error, Write};

use console::{style, Term};
use dialoguer::{Confirm, Input};

fn main() -> Result<(), Error> {
    let term = Term::stdout();

    term.clear_screen()?;

    let name: String = Input::new()
        .with_prompt(format!("What's your {}?", style("cool name").red()))
        .interact_text()
        .unwrap();

    let proceed = Confirm::new()
        .with_prompt("So, do you have courage?")
        .interact()
        .unwrap();

    match proceed {
        true => println!(
            "{} wants to fight ender dragon. {}!!!",
            name,
            style("A TRUE WARRIOR").green().blink()
        ),
        false => println!("{} is {}.", name, style("scared lol").magenta()),
    }

    // This is all env variables.
    let vars: Vec<(String, String)> = std::env::vars().collect();
    println!(
        "btw {}, you have {:?} environment variables.",
        name,
        vars.len()
    );

    Ok(())
}
