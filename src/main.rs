// https://youtu.be/jVpsLMCIB0Y?t=149 vigenere explanation
use std::env;
use std::fs;

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() == 1 {
        eprintln!("No flags provided, run v1 -h for options");
        std::process::exit(1);
    }

    let mut key: Vec<char> = vec![]; // key to encode text
    let mut readfile: Vec<char> = vec![]; // file to encode
    let mut alphabet: Vec<char> = vec![]; // file with alphabet to create board
    let mut writefile: String = String::new(); // file to write encoded string to

    let mut r: bool = false;
    let mut w: bool = false; // write file
    let mut e: bool = false; // encrypt
    let mut d: bool = false;
    let mut a: bool = false; // alphabet
    let mut k: bool = false; // key
    let mut p: bool = false; // print board

    let mut i = 1;

    while i < input.len() {
        match input[i].as_str() {
            "-e" | "--encode" => {
                e = true;
            }

            "-d" | "--decode" => {
                d = true;
            }

            "-r" | "--read" => {
                readfile = fs::read_to_string(input[i + 1].clone()).expect("Error reading target encode file").to_lowercase().chars().collect();
                r = true;
                i += 1;
            }

            "-w" | "--write" => {
                writefile = input[i + 1].clone();
                w = true;
                i += 1;
            }

            "-a" | "--alphabet" => {
                alphabet = fs::read_to_string(input[i + 1].clone()).expect("Error reading alphabet file").to_lowercase().trim_end().chars().collect();
                for x in 0..alphabet.len() {
                    for y in 0..alphabet.len() {
                        if alphabet[x] == alphabet[y] && x != y {
                            eprintln!("Can not have multiple instances of the same letter in alphabet file ({})", alphabet[x]);
                            std::process::exit(1);
                        }
                    }
                }
                a = true;
                i += 1;
            }

            "-k" | "--key" => {
                key = input[i + 1].chars().collect();
                k = true;
                i += 1;
            }

            "-p" | "--print" => {
                p = true;
            }

            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            } // print help

            _ => {
                eprintln!("Unknown flag {}", input[i]);
                std::process::exit(1);}
        }
        i += 1;
    }

    // validate inputs are not missing and nothing clashes
    check_input(r, w, e, d, a, k);

    // remove non alphabet characters

    // get rid of this bs
    readfile.retain(|c| *c == 'a' || *c == 'b' || *c ==  'c' || *c == 'd' || *c == 'e' || *c == 'f' || *c == 'g' || *c == 'h' || *c == 'i' || *c == 'j' || *c == 'k' || *c == 'l' || *c == 'm' || *c == 'n' || *c == 'o' || *c == 'p' || *c == 'q' || *c == 'r' || *c == 's' || *c == 't' || *c == 'u' || *c == 'v' || *c == 'w' || *c == 'x' || *c == 'y' || *c == 'z');

    let keystring: Vec<char> = init_key(key, &readfile).chars().collect();

    let board = init_board(alphabet);

    if e == true {
        fs::write(writefile, encode(keystring, readfile, &board)).expect("error writing to destination");
    }
    else if d == true {
        fs::write(writefile, decode(keystring, readfile, &board)).expect("error writing to destination");
    }

    if p {
        print_board(&board);
    }
}

/// Creates a string of length readfile.len() composed of the value key repeating
fn init_key(key: Vec<char>, readfile: &Vec<char>) -> String {
    let mut keystring = String::new();
    let mut index: usize;
    for x in 0..readfile.len() {
            index = x % key.len();
            keystring.push(key[index]);
    }

    return keystring;
}

/// Uses the alphabet provded by the user to create the board that will be used for encoding or decoding
fn init_board(alphabet: Vec<char>) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new(); // set board to be two dimensional of undefined height or length
    board.push(alphabet); // init first line with alphabet provided in file

    let len = board[0].len(); // length of first row in board

    for x in 1..len { // creates new line in grid and moves first letter to the end of the line
        // copies first character to chrctr, pushes chrctr, removes first value (index 0)
        board.push(board[x - 1].clone()); // set current line equal to previous line
        let ch = board[x].remove(0); // removes first character and assigns it to ch
        board[x].push(ch); // first character gets sent to the end of the line
    }

    return board;
}

/// Takes in a string of plain text and uses a key and alphabet to encode the text
fn encode(key: Vec<char>, plaintext: Vec<char>, board: &Vec<Vec<char>>) -> String {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut encoded_str = String::new(); // this will hold the encoded of the string

    // start of encoding logic
    for charx in 0..plaintext.len() {
        // finds letter on the x axis
        for char1 in 0..board[0].len() {
            if key[charx] == board[0][char1] {
                x = char1;
                break;
            }
        }
        // finds letter on the y axis
        for char2 in 0..board.len() {
            if plaintext[charx] == board[char2][0] {
                y = char2;
                break;
            }
        }
        encoded_str.push(board[y][x]);
    }

    return encoded_str;
}

/// Takes in a string of encoded text and uses a known key and alphabet to decode the text
fn decode(key_string: Vec<char>, encoded_str: Vec<char>, board: &Vec<Vec<char>>) -> String {
    //start with key and match key[index] with first character with any of boards collumns board[0][x]
    //go to character that matches the one in the encoded_str
    //use x value to find the original letter
    let mut decoded_str = String::new();
    let mut y: usize;
    let mut x: usize;
    for i in 0..encoded_str.len() {
        y = 0;
        x = 0;
        while key_string[i] != board[y][0] {
            y += 1;
        } // stops at the y index in the board of the char that matches encoded_str[i]

        while encoded_str[i] != board[y][x] {
            x += 1;
        }

        decoded_str.push(board[0][x]);
    }

    return decoded_str;
}

/// Prints the previously initialized board to the terminal
fn print_board(board: &Vec<Vec<char>>) {
    for y in board {
        for x in y {
            print!("{x} ");
        }
        println!("");
    }
}

/// Ensures all needed flags are provided by the user
fn check_input(r: bool, w: bool, e: bool, d: bool, a: bool, k: bool) {
    if e ==  d  {
        if e == true {
            eprintln!("Can not use the --encode and --decode flag together");
        }

        if e == false {
            eprintln!("No --encode or --decode flag set");
        }
    }

    if r == false {
        eprintln!("No file provided to be read. Use -r or --read to assign a target");
        std::process::exit(1);
    }

    if w == false {
        eprintln!("No write destination provided. Use -w or --write to assign a target");
        std::process::exit(1);
    }

    if a == false {
        eprintln!("No alphabet provided. Use -a or --alphabet to assign an alphabet text file");
        std::process::exit(1);
    }

    if k == false {
        eprintln!("No key provided. Use -k or --key to assign a key");
        std::process::exit(1);
    }
}

/// Prints help/options
fn print_help() {
    println!(
"Command line vigenere cypher tool

Usage: v1 -r READ.txt -w WRITE.txt -a ALPHABET.txt -k KEY [OPTIONS...]

Options:
    -a, --alphabet  ALPHABET.txt    Alphabet that will be used to encode/decode the file
    -k, --key       KEY             Key that will be used to encode/decode the file
    -r, --read      READ.txt        Target file for encoding/decoding
    -w, --write     WRITE.txt       Destination for output

    -e, --encode                    Encode read file
    -d, --decode                    Decode read file

    -p, --print                     Prints the board generated for encoding/decoding

    -h, --help                      Displays usage options

Malik Alshaer malik.h.alshaer@gmail.com");
}
