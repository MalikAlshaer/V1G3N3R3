// https://youtu.be/jVpsLMCIB0Y?t=149 vigenere explanation
use std::env;
use std::fs;

fn main() {
    let input: Vec<String> = env::args().collect();

    let mut shift: Vec<char> = vec![]; // board modifier
    let mut key: Vec<char> = vec![]; // key to encrypt text
    let mut readfile: Vec<char> = vec![]; // file to encrypt
    let mut writefile: String = String::new(); // file to write encrypted string in

    let mut encryption: bool = true;
    let mut i = 1;

    while i < input.len() {
        match input[i].as_str() {
            "-e" => {
                encryption = true;
                readfile = fs::read_to_string(input[i + 1].clone()).expect("error reading file").to_lowercase().chars().collect();
                i += 1;
            }
            "-d" => {
                encryption = false;
                readfile = fs::read_to_string(input[i + 1].clone()).expect("error reading file").to_lowercase().chars().collect();
                i += 1;
            }
            "-k" | "--key" => {
                key = input[i + 1].chars().collect();
                i += 1;
            }
            "-s" | "--shift" => {
                shift = input[i + 1].chars().collect();
                i += 1;
            }
            "-w" | "--write" => {
                writefile = input[i + 1].clone();
                i += 1;
            }
            _ => {
                eprintln!("Unknown flag {}", input[i]);
                std::process::exit(1);}
        }
        i += 1;
    }

    // remove non alphabet characters
    readfile.retain(|c| *c == 'a' || *c == 'b' || *c ==  'c' || *c == 'd' || *c == 'e' || *c == 'f' || *c == 'g' || *c == 'h' || *c == 'i' || *c == 'j' || *c == 'k' || *c == 'l' || *c == 'm' || *c == 'n' || *c == 'o' || *c == 'p' || *c == 'q' || *c == 'r' || *c == 's' || *c == 't' || *c == 'u' || *c == 'v' || *c == 'w' || *c == 'x' || *c == 'y' || *c == 'z');

    // following chunk of code creates a
    // string of length plaintext.len()
    // composed of the value key repeating
    let mut keystring = String::new();
    let mut index: usize;
    for x in 0..readfile.len() {
            index = x % key.len();
            keystring.push(key[index]);
    }

    let keystring: Vec<char> = keystring.chars().collect();


    let board = init_board(shift);

    if encryption == true {
        fs::write(writefile, encrypt(keystring, readfile, &board)).expect("error writing to destination");
    }
    else {
        fs::write(writefile, decrypt(keystring, readfile, &board)).expect("error writing to destination");
    }

    //_print_board(&board);
}

fn init_board(shift: Vec<char>) -> Vec<Vec<char>> {
    let mut board: Vec<Vec<char>> = Vec::new(); // set board to be two dimensional of undefined height or length
    board.push(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']);

    for x in 0..shift.len() { // init first line
        board[0].retain(|&z| z != shift[x]);
        board[0].insert(x, shift[x]);
    }

    let len = board[0].len(); // use updated length = 26 + length of keystr
    let mut chrctr: char;

    for x in 1..len { // creates new line in grid and moves first letter to the end of the line
        // copies first character to chrctr, pushes chrctr, removes first value (index 0)
        board.push(board[x - 1].clone()); // set current line equal to previous line
        chrctr = board[x][0]; // find a way to complete the process without this variable
        board[x].push(chrctr); // copy of first character gets sent to the end of the line
        board[x].remove(0); // remove first value (index 0)
    }

    return board;
}

fn encrypt(key: Vec<char>, plaintext: Vec<char>, board: &Vec<Vec<char>>) -> String {
    println!("\n");

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut encrypted_str = String::new(); // this will be the encrypted version of the string

    // start of encryption logic
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
        encrypted_str.push(board[y][x]);
    }
    //println!("{}\n", encrypted_str);
    return encrypted_str;
}

fn decrypt(key_string: Vec<char>, encrypted_str: Vec<char>, board: &Vec<Vec<char>>) -> String {
    //start with key and match key[index] with first character with any of boards collumns board[0][x]
    //go to character that matches the one in the encrypted_str
    //use x value to find the original letter
    let mut unencrypted = String::new();
    let mut y: usize;
    let mut x: usize;
    for i in 0..encrypted_str.len() {
        y = 0;
        x = 0;
        while key_string[i] != board[y][0] {
            y += 1;
        } // stops at the y index in the board of the char that matches encrypted_str[i]

        while encrypted_str[i] != board[y][x] {
            x += 1;
        }

        unencrypted.push(board[0][x]);
    }

    //println!("{}", unencrypted);
    return unencrypted;
}

fn _print_board(board: &Vec<Vec<char>>) {
    for y in board {
        for x in y {
            print!("{x}");
        }
        println!("");
    }
}
