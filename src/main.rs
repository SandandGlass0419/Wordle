use std::io;
use colored::{ColoredString, Colorize};

fn main() {
    let word = "FERRIS"; // upper

    let mut state = State::new(5);
    let won = turn(word, &mut state);

    println!("{} Word: {}", if won {"Win!"} else {"Lose!"} ,word);
}

fn turn(word: &str, state: &mut State) -> bool {
    if state.attempts > state.tries { return false }

    let guess = read_guess(&format!("Guess {}/{}, {} letters", state.attempts, state.tries, word.chars().count()));

    let Ok(colored_guess) = color_guess(word, &guess) else {
        println!("Bad guess length!");

        return turn(word, state)
    };

    state.update(&colored_guess);
    state.attempts += 1;

    print_letters(&colored_guess);
    state.progress_print();

    if verify(&colored_guess) { true } else { turn(word, state) }
}

fn read_guess(message: &str)-> String {
    println!("{}", message);

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("failed to read");

    buf.trim().to_uppercase()
}

fn color_guess(word: &str, guess: &str) -> Result<Vec<Letter>, String> {
    validate_guess(word, guess)?;

    let queue = queue_indexes(word, guess)?;

    let mut colored_guess = guess.chars().map(|c| Letter::Gray(c)).collect();
    color(&mut colored_guess, &queue.0, Letter::GREEN);
    color(&mut colored_guess, &queue.1, Letter::YELLOW);

    Ok(colored_guess)
}

fn validate_guess(word: &str, guess: &str) -> Result<(), String> {
    if word.chars().count() != guess.chars().count() {
        return Err(String::from("bad"))
    }

    Ok(())
}

fn queue_indexes(word: &str, guess: &str) -> Result<(Vec<usize>, Vec<usize>), String> {
    let mut green_queue = Vec::new();
    let mut yellow_queue = Vec::new();

    for i in 0..guess.chars().count() {
        if green(nth(word, i)?, nth(guess, i)?) {
            green_queue.push(i);
        }
        else if yellow(word, nth(guess, i)?) {
            yellow_queue.push(i);
        }
    }

    Ok((green_queue, yellow_queue))
}

fn nth(str: &str, n: usize) -> Result<char, String> {
    str.chars().nth(n).ok_or(String::from("bad"))
}

fn green(word: char, guess: char) -> bool {
    word == guess
}

fn yellow(word: &str, guess: char) -> bool {
    word.contains(guess)
}

fn color(letters: &mut Vec<Letter>, queue: &[usize], color: Letter) {
    for &i in queue {
        letters[i] = letters[i].to_color(color);
    }
}

fn print_letters(letters: &Vec<Letter>) {
    for &l in letters {
        print!("{}", l.as_bg_colored());
    }

    println!();
}

fn verify(colored_guess: &Vec<Letter>) -> bool {
    for &l in colored_guess {
        let Letter::Green(_) = l else { return false };
    }

    true
}

struct State {
    attempts: i32,
    tries: i32,
    progress: Vec<Letter>
}

impl State {
    const CHAR_SET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn new(tries: i32) -> State {
        State {
            attempts: 1,
            tries: tries,
            progress: State::CHAR_SET.chars().map(|c| Letter::White(c)).collect(),
        }
    }

    fn update(&mut self, guess: &Vec<Letter>) {
        for &l in guess {
            if let Some(_mut_l) = self.progress_find(l) {
                *_mut_l = _mut_l.to_color(l);
            }       
        }
    }

    fn progress_find(&mut self, letter: Letter) -> Option<&mut Letter> {
        self.progress.iter_mut().find(|l| l.unwrap() == letter.unwrap())
    }

    fn progress_print(&self) {  
        for &l in &self.progress {
            print!("{} ", l.as_fg_colored());
        }

        println!();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Letter {
    Gray(char),
    Yellow(char),
    Green(char),
    White(char),
}

impl Letter {
    const GRAY: Letter = Letter::Gray(char::MIN);
    const YELLOW: Letter = Letter::Yellow(char::MIN);
    const GREEN: Letter = Letter::Green(char::MIN);
    const WHITE: Letter = Letter::White(char::MIN);

    fn unwrap(&self) -> char {
        match self {
            Letter::Gray(c) => *c,
            Letter::Yellow(c) => *c,
            Letter::Green(c) => *c,
            Letter::White(c) => *c,
        }
    }

    fn to_color(&self, color: Letter) -> Letter {
        match color {
            Letter::Gray(_) => Letter::Gray(self.unwrap()),
            Letter::Yellow(_) => Letter::Yellow(self.unwrap()),
            Letter::Green(_) => Letter::Green(self.unwrap()),
            Letter::White(_) => Letter::White(self.unwrap()),
        }
    }

    fn as_bg_colored(&self) -> ColoredString {
        match self {
            Letter::Gray(c) => c.to_string().on_bright_black().bold(),
            Letter::Yellow(c) => c.to_string().on_yellow().bold(),
            Letter::Green(c) => c.to_string().on_green().bold(),
            Letter::White(c) => c.to_string().on_white().bold(),
        }
    }

    fn as_fg_colored(&self) -> ColoredString {
        match self {
            Letter::Gray(c) => c.to_string().bright_black(),
            Letter::Yellow(c) => c.to_string().yellow().bold(),
            Letter::Green(c) => c.to_string().green().bold(),
            Letter::White(c) => c.to_string().white(),
        }
    }
}