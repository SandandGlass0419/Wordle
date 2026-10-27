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

    let guess = read_guess(&format!("Guess {}/{}, {} letters", state.attempts, state.tries, len(word)));

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

    let mut colored_guess: Vec<Letter> = guess.chars().map(|c| Letter::new(c, Color::Gray)).collect();
    
    for (i, l) in colored_guess.iter_mut().enumerate() {
        color_green(nth(word, i)?, l);

        color_yellow(word, l);
    }


    Ok(colored_guess)
}

fn validate_guess(word: &str, guess: &str) -> Result<(), String> {
    if len(word) != len(guess) {
        return Err(String::from("bad"))
    } 
    
    Ok(())
}

fn color_green(word: char, guess: &mut Letter) {
    if word == guess.char {
        guess.color(Color::Green);
    }
}

fn color_yellow(word: &str, guess: &mut Letter) {
    if word.contains(guess.char) {
        guess.color(Color::Yellow)
    }
}

fn verify(guess: &Vec<Letter>) -> bool {
    guess.iter().all(|l| l.color == Color::Green)
}

fn nth(word: &str, n: usize) -> Result<char, String> {
    word.chars().nth(n).ok_or(String::from("bad"))
}

fn len(word: &str) -> usize {
    word.chars().count()
}

fn print_letters(letters: &Vec<Letter>) {
    for l in letters {
        print!("{}", l.as_bg_colored());
    }

    println!();
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
            progress: State::CHAR_SET.chars().map(|c| Letter::new(c, Color::White)).collect(),
        }
    }

    fn update(&mut self, guess: &Vec<Letter>) {
        for l in guess {
            if let Some(_mut_l) = self.progress_find(&l) {
                _mut_l.color(l.color);
            }       
        }
    }

    fn progress_find(&mut self, letter: &Letter) -> Option<&mut Letter> {
        self.progress.iter_mut().find(|l| l.char == letter.char)
    }

    fn progress_print(&self) {  
        for l in &self.progress {
            print!("{} ", l.as_fg_colored());
        }

        println!();
    }
}

struct Letter {
    char: char,
    color: Color,
}

impl Letter {
    fn new(char: char, color: Color) -> Letter {
        Letter {
            char: char,
            color: color,
        }
    }

    fn color(&mut self, color: Color) {
        if color.lvl() > self.color.lvl() {
            self.color = color;
        }
    }

    fn as_bg_colored(&self) -> ColoredString {
        match self.color {
            Color::Gray => self.char.to_string().on_bright_black().bold(),
            Color::Yellow => self.char.to_string().on_yellow().bold(),
            Color::Green => self.char.to_string().on_green().bold(),
            Color::White => self.char.to_string().on_white().bold(),
        }
    }

    fn as_fg_colored(&self) -> ColoredString {
        match self.color {
            Color::Gray => self.char.to_string().bright_black(),
            Color::Yellow => self.char.to_string().yellow().bold(),
            Color::Green => self.char.to_string().green().bold(),
            Color::White => self.char.to_string().white(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Color {
    Gray,
    Yellow,
    Green,
    White,
}

impl Color {
    const GRAY_LVL: u8 = 1;
    const YELLOW_LVL: u8 = 2;
    const GREEN_LVL: u8 = 3;
    const WHITE_LVL: u8 = 0;

    fn lvl(&self) -> u8 {
        match self {
            Color::Gray => Color::GRAY_LVL,
            Color::Yellow => Color::YELLOW_LVL,
            Color::Green => Color::GREEN_LVL,
            Color::White => Color::WHITE_LVL,
        }
    }
}