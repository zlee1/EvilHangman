use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::io::{stdin,stdout};
use rand::Rng;

// The word the user is guessing will have N characters.
const N: usize = 6;

// Remove all words from initial list that do not have N characters.
fn initialize_list(n: usize, words: &mut Vec<String>) -> Vec<String> {
    let mut trimmed = Vec::<String>::new();

    for s in words {
        // Check if number of characters in word is N.
        if s.chars().count() == n {
            trimmed.push(s.to_string());
        }
    }

    // Return the trimmed list of words.
    trimmed
}

// Let the user input a character for their turn.
fn turn() -> char {
    // Taking user input
    let mut g = String::new();
    print!("Enter your guess: ");
    let _=stdout().flush();
    stdin().read_line(&mut g).expect("Did not enter a correct string");
    if let Some('\n')=g.chars().next_back() {
        g.pop();
    }
    if let Some('\r')=g.chars().next_back() {
        g.pop();
    }

    // Return the input character
    g.chars().next().unwrap()
}

// Remove words from the list that contain the letter just guessed
fn remove_char(c: char, words: &mut Vec<String>) -> Vec<String>{
    let mut trimmed = Vec::<String>::new();
    for s in words{
        // Only include words that do not contain character c
        if !s.contains(c){
            trimmed.push(s.to_string());
        }
    }

    // Return trimmed list of words.
    trimmed
}

// Print the current board.
fn print_board(arr:[char;N]){
    println!();
    for i in arr.iter(){
        print!("{} ", i.to_string());
    }
    println!();
}

// Return a random word from the list of available words.
fn pick_word(words: &mut Vec<String>) -> String{
    words[rand::thread_rng().gen_range(0, words.len())].to_string()
}

// Update the board to display guessed characters properly
fn update_board(g: char, mut arr: [char;N], ex_word: String) -> [char;N]{
    for (i, c) in ex_word.chars().enumerate(){
        if c == g {
            arr[i] = g;
        }
    }
    arr
}

// Update the word list to ensure that all words are possible for the current board.
fn updated_board_words(arr: [char;N], words: &mut Vec<String>) -> Vec<String>{
    let mut n_words = Vec::<String>::new();
    for i in words {
        let mut matches = true;
        for (j, c) in i.chars().enumerate(){
            // If letter has been guessed and is not the same as character in that
            // position of the word, this does not match the board and cannot be included.
            if arr[j] != '_' && arr[j] != c{
                matches = false;
            }
        }
        if matches {
            n_words.push(i.to_string());
        }
    }
    n_words
}

// Check if the user has won.
fn check_win(arr: [char;N]) -> bool {
    let mut won = true;
    // A win is when all characters are guessed.
    for i in arr.iter(){
        if i == &'_' {
            won = false;
        }
    }
    won
}

fn run() -> io::Result<()> {
    // Read the dictionary file into a vector.
    let mut words = Vec::<String>::new();
    let file = File::open("data/dictionary.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        words.push(line.unwrap());
    }

    // Initialize list of words, board, and variables
    words = initialize_list(N, &mut words);
    let mut board:[char;N] = ['_';N];
    print_board(board);

    //Represents whether the game is over.
    let mut over = false;
    // Represents the number of tries remaining.
    let mut turns = 10;

    // Run until the game ends
    while !over {
        let g = turn();
        // Remove the words that have character g.
        let n_words = remove_char(g, &mut words);

        // If there are no remaining words without character g,
        // pick a random word from the list, place character g
        // in the correct spots on the board, and then update the list.
        if n_words.len() == 0 {
            let ex_word = pick_word(&mut words);
            board = update_board(g, board, ex_word);
            words = updated_board_words(board, &mut words);
        }else{
            // If there are words without character g,
            // the user has "guessed incorrectly", and all words with
            // character g have been removed from the list.
            words = n_words;
            turns -= 1;
            println!("Remaining tries: {:?}", turns);
        }

        print_board(board);

        // If the user has run out of turns, the game is over.
        if turns == 0 {
            over = true;
            // Select a random word from the list, as all of them should match the board.
            println!("You lost. The word was: {}", pick_word(&mut words));
        }else if check_win(board) {
            over = true;
            println!("You won!");
        }
    }

    Ok(())
}

fn main() {
    // Run the game.
    let _ = run();
}
