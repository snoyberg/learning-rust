extern crate rand;
use rand::distributions::{Range, IndependentSample};
use std::io::{Write, BufRead};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Game(State);

#[derive(Debug, PartialEq, Eq)]
pub enum Response {
    AnswerIsHigher(Game),
    AnswerIsLower(Game),
    OutsideRange(Game),
    Correct,
    YouLose(isize),
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    answer: isize,
    low: isize,
    high: isize,
    guesses_left: usize,
}

impl Game {
    pub fn new() -> Game {
        Game::new_internal(10, 1, 100)
    }

    // Split off in case we wanted a new_with function with different
    // values
    fn new_internal(guesses: usize, low: isize, high: isize) -> Game {
        Game(State {
            low: low,
            high: high,
            answer: Range::new(low, high + 1).ind_sample(&mut rand::thread_rng()),
            guesses_left: guesses,
        })
    }

    pub fn ask_guess(&self, in_: &mut BufRead, out: &mut Write) -> Result<isize, std::io::Error> {
        let msg = self.to_string();

        loop {
            // Would it be more efficient/idiomatic to put buffer
            // outside the loop and call buffer.clear()?
            let mut buffer = String::new();

            out.write_all(msg.as_bytes())?;
            out.flush()?;
            in_.read_line(&mut buffer)?;

            let guess_str = buffer.trim();

            match guess_str.parse::<isize>() {
                Ok(guess) => {
                    return Ok(guess);
                },
                Err(err) => {
                    writeln!(
                        out,
                        "Invalid guess '{}' ({}), try again",
                        guess_str,
                        err,
                    )?;
                },
            }
        };
    }

    pub fn ask_play_again(in_: &mut BufRead, out: &mut Write) -> Result<bool, std::io::Error> {
        loop {
            write!(out, "Play again (y/n)? ")?;
            out.flush()?;

            let mut buffer = String::new();
            in_.read_line(&mut buffer)?;

            match buffer.trim() {
                "y" => return Ok(true),
                "n" => return Ok(false),
                _ => writeln!(out, "Invalid input, try again\n")?,
            }
        }
    }

    // This seems a bit cheaky, not sure if it's the right Rust
    // approach at all. We pass in a Game value here, disallowing it
    // from being reused, and then only return a new Game in the cases
    // of the Response which allow the game to continue.
    pub fn guess(self, guess: isize) -> Response {
        #[cfg(test)]
        self.check_invariants();

        let mut state = self.0;

        if guess < state.low || guess > state.high {
            Response::OutsideRange(Game(state))
        } else if guess == state.answer {
            Response::Correct
        } else if state.guesses_left <= 1 {
            Response::YouLose(state.answer)
        } else if guess < state.answer {
            state.guesses_left -= 1;
            state.low = guess + 1;
            Response::AnswerIsHigher(Game(state))
        } else {
            state.guesses_left -= 1;
            state.high = guess - 1;
            Response::AnswerIsLower(Game(state))
        }
    }

    pub fn print_response(response: Response, out: &mut std::io::Write) -> Result<Option<Game>, std::io::Error> {
        Ok(match response {
            Response::AnswerIsHigher(game) => {
                writeln!(out, "The answer is higher, try again")?;
                Some(game)
            },
            Response::AnswerIsLower(game) => {
                writeln!(out, "The answer is lower, try again")?;
                Some(game)
            },
            Response::OutsideRange(game) => {
                writeln!(out, "Your guess was out of range, try again")?;
                Some(game)
            },
            Response::Correct => {
                writeln!(out, "Congratulations! You win!")?;
                None
            },
            Response::YouLose(answer) => {
                writeln!(out, "You lose, the answer was {}", answer)?;
                None
            },
        })
    }

    #[cfg(test)]
    fn check_invariants(&self) {
        assert!(self.0.low < self.0.answer);
        assert!(self.0.answer < self.0.high);
        assert!(self.0.guesses_left > 0);
    }
}

// I have a strong feeling that I'm completely misusing the Display
// trait
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "You have {} guesses left.\n\nGuess a number between {} and {}: ",
            self.0.guesses_left,
            self.0.low,
            self.0.high,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invariants() {
        let game = Game::new();
        game.check_invariants();
        match game.guess(0) {
            Response::OutsideRange(_) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn correct_guess() {
        let game = Game(State{
            answer: 5,
            low: 4,
            high: 6,
            guesses_left: 1,
        });

        assert_eq!(game.guess(5), Response::Correct);
    }

    #[test]
    fn ask_play_again() {
        let mut input: &[u8] = &"y\n".as_bytes()[..];
        assert!(Game::ask_play_again(&mut input, &mut std::io::sink()).unwrap());

        let mut input: &[u8] = &"n\n".as_bytes()[..];
        assert!(!Game::ask_play_again(&mut input, &mut std::io::sink()).unwrap());

        let mut input: &[u8] = &"x\ndjfjfa\nyn\n\nn\n".as_bytes()[..];
        assert!(!Game::ask_play_again(&mut input, &mut std::io::sink()).unwrap());
    }

    #[test]
    fn ask_guess() {
        let game = Game(State{
            answer: 1,
            low: 0,
            high: 2,
            guesses_left: 1,
        });

        let mut input: &[u8] = &"y\n\n5\n".as_bytes()[..];
        let guess = game.ask_guess(&mut input, &mut std::io::sink()).unwrap();
        assert_eq!(guess, 5);
    }
}
