use std::io;
use clap::{Arg, ArgMatches, command, value_parser};
use std::collections::HashSet;
use colored::{ColoredString, Colorize};

fn main() {
    let matches = match_args();

    let raw;

    let arg_tries = *matches.get_one::<u32>("tries").unwrap_or(&7);
    let arg_len = *matches.get_one::<u32>("len").unwrap_or(&fastrand::u32(4..=8));
    let arg_word = match matches.get_one::<String>("word") {
        Some(s) => {
            raw = load_dict(len(s) as u32);
            s.to_lowercase()
        }
        None => {
            raw = load_dict(arg_len);
            raw[fastrand::usize(0..raw.len())].clone()
        }
    };
    
    let dict = HashSet::from_iter(raw);

    let mut state = State::new(arg_tries);
    let won = turn(&dict, &arg_word, &mut state);

    println!("{} Word: {}", if won {"Win!"} else {"Lose!"}, arg_word.to_uppercase());
}

fn match_args() -> ArgMatches {
    let tries = Arg::new("tries")
        .short('t')
        .long("tries")
        .value_parser(value_parser!(u32).range(1..))
        .help("Set tries to get in game.");

    let word = Arg::new("word")
        .short('w')
        .long("word")
        .value_parser(|s: &str|
            match len(s) {
                4..=8 => Ok(s.to_string()),
                _ => Err(format!("'{s}' is not 4-8 letters.")),
            }
        )
        .help("Set target word manually. (overrides dict)");

    let len = Arg::new("len")
        .short('l')
        .long("len")
        .value_parser(value_parser!(u32).range(4..=8))
        .conflicts_with("word")
        .help("Select a random word with specified length.");

    command!()
        .arg(tries)
        .arg(word)
        .arg(len)
        .get_matches()
}

fn load_dict(len: u32) -> Vec<String> {
    match len {
        4 => include_str!("../dict/4.txt"),
        5 => include_str!("../dict/5.txt"),
        6 => include_str!("../dict/6.txt"),
        7 => include_str!("../dict/7.txt"),
        8 => include_str!("../dict/8.txt"),
        _ => panic!("dict load failed"),
    }
    .split('\n').map(|s| s.trim().to_string()).collect()
}

fn turn(dict: &HashSet<String>, word: &str, state: &mut State) -> bool {
    if state.attempt > state.tries { return false }

    let guess = read_guess(&format!("Guess {}/{}, {} letters", state.attempt, state.tries, len(word)));

    if let Err(err) = validate_guess(dict, word, &guess) {
        println!("{err}");

        return turn(dict, word, state)
    }

    let colored_guess = color_guess(word, &guess);

    state.update(&colored_guess);
    state.attempt += 1;

    print_letters(&colored_guess);
    state.progress_print();

    if verify(&colored_guess) { true } else { turn(dict, word, state) }
}

fn read_guess(message: &str) -> String {
    println!("{}", message);

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("read failed");

    buf.trim().to_lowercase()
}

fn color_guess(word: &str, guess: &str) -> Vec<Letter> {
    let mut colored_guess: Vec<Letter> = guess.chars().map(|c| Letter::new(c, Color::Gray)).collect();
    
    for (i, l) in colored_guess.iter_mut().enumerate() {
        color_green(word.chars().nth(i), l);

        color_yellow(word, l);
    }

    colored_guess
}

fn validate_guess(dict: &HashSet<String>, word: &str, guess: &str) -> Result<(), String> {
    if word == guess {  // custom word not in dict
        Ok(())
    } else if len(word) != len(guess) {
        Err(String::from("Wrong letter count!"))
    } else if !dict.contains(guess) {
        Err(String::from("Not a word!"))
    } else {
        Ok(())
    }
}

fn color_green(word: Option<char>, guess: &mut Letter) {
    if let Some(c) = word && c == guess.char {
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
    attempt: u32,
    tries: u32,
    progress: Vec<Letter>
}

impl State {
    const CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyz";

    fn new(tries: u32) -> State {
        State {
            attempt: 1,
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
        let s = self.char.to_uppercase().to_string();
        
        match self.color {
            Color::Gray => s.on_bright_black().bold(),
            Color::Yellow => s.on_yellow().bold(),
            Color::Green => s.on_green().bold(),
            Color::White => s.on_white().bold(),
        }
    }

    fn as_fg_colored(&self) -> ColoredString {
        let s = self.char.to_uppercase().to_string();
        
        match self.color {
            Color::Gray => s.bright_black(),
            Color::Yellow => s.yellow().bold(),
            Color::Green => s.green().bold(),
            Color::White => s.white(),
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