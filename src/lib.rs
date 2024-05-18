use std::io;
use rand::{self, Rng};
use std::{thread, time};

use itertools::Itertools;
use ordinal::Ordinal;

const MINIMUM_PATTERN_LENGTH: usize = 4;
const MAXIMUM_PATTERN_LENGTH: usize = 64;
const MINIMUM_PLAYER_COUNT: usize = 1;
const MAXIMUM_PLAYER_COUNT: usize = 8;

const CHARACTERS: [char; MAXIMUM_PATTERN_LENGTH] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','-','_'];

enum InitialGuessError {
    WrongLength,
    NotADigit(char),
    NotInPattern(char),
}


pub struct Game {
    players: Vec<Player>,
    pattern_length: usize,
}

impl Game {
    pub fn new() -> Self {
        let player_count = loop {
            println!("How many players are playing: (Can be any whole number between {} and {})", MINIMUM_PLAYER_COUNT, MAXIMUM_PLAYER_COUNT);

            let mut input = String::new();
    
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");
    
            let input = input.trim();

            match input.parse::<isize>() {
                Ok(num) => if !(MINIMUM_PLAYER_COUNT..=MAXIMUM_PLAYER_COUNT).contains(&(num as usize)) {
                    println!("\nPlease try again, {}, is not between {} and {},\n", num, MINIMUM_PLAYER_COUNT, MAXIMUM_PLAYER_COUNT);
                } else {
                    break num as usize
                },
                Err(_) => println!("\nPlease try again, {}, is not a whole number,\n", input),
            }

            thread::sleep(time::Duration::from_millis(250));
        };

        println!("\nThe player count is: {}.\n", player_count);


        let pattern_length = loop {
            println!("How many characters long the patterns should be: (Can be any whole number between {} and {})", MINIMUM_PATTERN_LENGTH, MAXIMUM_PATTERN_LENGTH);

            let mut input = String::new();
    
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            let input = input.trim();

            match input.parse::<isize>() {
                Ok(num) => if !(MINIMUM_PATTERN_LENGTH..=MAXIMUM_PATTERN_LENGTH).contains(&(num as usize)) {
                    println!("\nPlease try again, {}, is not between {} and {},\n", num, MINIMUM_PATTERN_LENGTH, MAXIMUM_PATTERN_LENGTH);
                } else {
                    break num as usize
                },
                Err(_) => println!("\nPlease try again, {}, is not a whole number,\n", input),
            }

            thread::sleep(time::Duration::from_millis(250));
        };

        println!("\nThe pattern length is: {}.\n", pattern_length);


        let mut players: Vec<Player> = Vec::with_capacity(player_count);

        for _ in 0..player_count {
            players.push(Player::new(pattern_length));
        }

        let mut game = Self {
            players,
            pattern_length,
        };

        game.init();

        game
    }


    pub fn init(&mut self) {
        let player_count: usize = self.players.len();

        for (player_number, player) in self.players.iter_mut().enumerate() {
            if player_count > 1 {
                println!("Player {},", player_number + 1);
            }

            loop {
                println!("Your pattern has the characters: {},", player.pattern.chars().sorted().collect::<String>());

                println!("Make your initial guess of the order the numbers are in:");

                let mut input = String::new();
            
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line!");

                let input = input.trim();

                match player.initial_guess(input) {
                    Ok(mut matches) => {
                        while matches == self.pattern_length {
                            player.pattern = Player::generate_pattern(self.pattern_length);
                            matches = player.count_matches();
                        }

                        break println!("")
                    },
                    Err(error) => match error {
                        InitialGuessError::WrongLength => println!("\nPlease try again, {}, does not match the length of the pattern,\n", input),
                        InitialGuessError::NotADigit(character) => println!("\nPlease try again, {}, is not a digit,\n", character),
                        InitialGuessError::NotInPattern(character) => println!("\nPlease try again, {}, is not in the pattern,\n", character),
                    },
                }

                thread::sleep(time::Duration::from_millis(250));
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }

    pub fn play(&mut self) {
        let mut move_number: usize = 1;
        let mut win_count: usize = 0;
        let player_count: usize = self.players.len();

        'outer: loop {
            println!("Move {}:\n", move_number);

            for (player_number, player) in self.players.iter_mut().enumerate() {
                if player.has_won > 0 {
                    continue;
                }

                if player_count > 1 {
                    println!("Player {},", player_number + 1);
                }

                loop {
                    thread::sleep(time::Duration::from_millis(250));

                    println!("Your current guess of the pattern is this: {},", player.guess);

                    let matches = player.count_matches();

                    println!("{} character{} of your guess match{} the pattern.\n", matches, if matches == 1 {""} else {"s"}, if matches == 1 {"es"} else {""});

                    println!("What are the positions of the two characters you want to swap: (2 whole numbers separated by a space)");

                    let mut input = String::new();
                
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line!");
                    
                    let input = input.trim();

                    let mut sliced_input = input.split_whitespace();
                    let count = sliced_input.clone().count();

                    if count != 2 {
                        println!("\nPlease try again, {}, is not the expected amount of arguments,\n", count);
                        continue;
                    }

                    let argument = sliced_input.next().expect("length is known to be 2, this element should exist");

                    let a_index = match argument.parse::<isize>() {
                        Ok(num) => if !(1..=player.guess.len()).contains(&(num as usize)) {
                            println!("\nPlease try again, {}, is not between 1 and {},\n", num, player.guess.len());
                            continue;
                        } else {
                            num as usize
                        },
                        Err(_) => {
                            println!("\nPlease try again, {}, is not a whole number,\n", argument);
                            continue;
                        },
                    };

                    let argument = sliced_input.next().expect("length is known to be 2, this element should exist");

                    let b_index = match argument.parse::<isize>() {
                        Ok(num) => if !(1..=player.guess.len()).contains(&(num as usize)) {
                            println!("Please try again, {}, is not between 1 and {},\n", num, player.guess.len());
                            continue;
                        } else {
                            num as usize
                        },
                        Err(_) => {
                            println!("Please try again, {}, is not a whole number,\n", argument);
                            continue;
                        },
                    };

                    println!("\nYou have swapped the {} and {} characters:", Ordinal(a_index).to_string(), Ordinal(b_index).to_string());

                    player.swap(a_index - 1, b_index - 1);

                    println!("{},", player.guess);

                    let matches = player.count_matches();

                    println!("{} characters of your guess match the pattern.\n", matches);

                    if matches == self.pattern_length {
                        win_count += 1;
                        player.has_won = win_count;

                        if player_count > 2 {
                            println!("You've won in {} place!\n", Ordinal(win_count).to_string());
                        } else if player_count == 2 && win_count == 1 {
                            println!("You've beat your opponent!\n");
                        } else {
                            println!("You've completed the puzzle!\n");
                        }
                    }

                    if win_count == player_count {
                        break 'outer;
                    }

                    break;
                }

                thread::sleep(time::Duration::from_millis(500));
            }

            move_number += 1;

            thread::sleep(time::Duration::from_millis(500));
        }

        println!("Game over after {} rounds!", move_number);
    }
}


#[derive(Clone)]
struct Player {
    pattern: String,
    guess: String,
    has_won: usize,
}

impl Player {
    fn new(pattern_length: usize) -> Self {
        Self {
            pattern: Self::generate_pattern(pattern_length),
            guess: String::with_capacity(pattern_length),
            has_won: 0,
        }
    }


    fn generate_pattern(length: usize) -> String {
        let mut pattern = String::with_capacity(length);

        let mut rng = rand::thread_rng();

        let mut chars = CHARACTERS[0..length].to_vec();

        for _ in 0..length {
            let i = rng.gen_range(0..chars.len());

            pattern.push(chars[i]);
            
            chars.remove(i);

            if chars.len() == 0 {
                chars = CHARACTERS[0..length].to_vec();
            }
        }

        pattern
    }


    fn initial_guess(&mut self, guess: &str) -> Result<usize, InitialGuessError> {
        if guess.len() != self.pattern.len() {
            return Err(InitialGuessError::WrongLength)
        }

        for character in guess.chars() {
            if !character.is_digit(10) {
                return Err(InitialGuessError::NotADigit(character))
            }

            if !self.pattern.contains(character) {
                return Err(InitialGuessError::NotInPattern(character))
            }
        }

        self.guess = String::from(guess);
        
        Ok(self.count_matches())
    }


    fn swap(&mut self, a_index: usize, b_index: usize) -> usize {
        let a = self.guess[a_index..=a_index].to_string();
        let b = self.guess[b_index..=b_index].to_string();

        self.guess.replace_range(a_index..=a_index, &b);
        self.guess.replace_range(b_index..=b_index, &a);

        self.count_matches()
    }

    fn count_matches(&self) -> usize {
        let mut matches: usize = 0;

        let mut ptg_chars = self.pattern.chars();
        let mut guess_chars = self.guess.chars();

        while let Some(ptg_char) = ptg_chars.next() {
            if let Some(guess_char) = guess_chars.next() {
                if ptg_char == guess_char {
                    matches += 1;
                }
            }
        }

        matches
    }
}