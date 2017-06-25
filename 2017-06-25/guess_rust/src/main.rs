extern crate guess;

use guess::Game;
use std::process;
use std::io::Write;

fn main() {
    match main_inner() {
        Ok(guess) => guess,
        Err(err) => {
            let mut stderr = std::io::stderr();
            writeln!(
                &mut stderr,
                "Error occurred while interacting with user: {}",
                err
            ).expect("Could not write to stderr");
            process::exit(1);
        },
    }
}

fn main_inner() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        let mut option_game = Some(Game::new());
        while let Some(game) = option_game.take() {

            let guess = game.ask_guess(&mut stdin.lock(), &mut stdout)?;
            let response = game.guess(guess);
            option_game = Game::print_response(response, &mut stdout)?;
        }

        if !Game::ask_play_again(&mut stdin.lock(), &mut stdout)? {
            return Ok(())
        };
    }
}
