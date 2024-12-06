mod game {
    pub mod game;
}
mod tokenisation {
    pub mod pgnparser;
    pub mod tokeniser;
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::time::Instant;
use flate2::read::GzDecoder;
use crate::tokenisation::pgnparser::parse_pgn_game;
use crate::tokenisation::tokeniser::{tokenise_to_bytes, tokenise_to_str};


// static mut TOTAL_MOVES: i64 = 0;


static TOTAL_GAMES: i64 = 57151024;

fn main() {
    let mut all_tokens: HashSet<String> = HashSet::new();
    let total_start_time = Instant::now();
    let mut total_parse_time = 0;
    let mut total_tokenise_time = 0;
    let mut parsed = 0;
    let mut i = 0;

    // Open the gzipped file
    let file = File::open("../../python/chessai/bestgames.gz").expect("Failed to open games file");
    let reader = BufReader::new(GzDecoder::new(file));
    // let mut reader = reader.lines();

    // Open a binary file for writing
    let mut tokens_file = File::create("../../python/chessai/tokens.bin").expect("Failed to open tokens file");

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let start_time = Instant::now();
        // println!("Game: {}", line);
        let _game = parse_pgn_game(&line);
        total_parse_time += start_time.elapsed().as_nanos() as i64;

        // let tokens = tokenise_to_str(&_game);
        // println!("Game: {}", _game.to_string());
        // println!("Tokens: {:?}", tokens);
        // all_tokens.extend(tokens);
        // println!("Total unique tokens: {}", all_tokens.len());
        let start_time = Instant::now();
        let _tokens = tokenise_to_bytes(&_game);
        total_tokenise_time += start_time.elapsed().as_nanos() as i64;
        // let tokenstr: String = String::from_(_tokens).unwrap();
        // println!("Tokens: {}", tokenstr);
        // for &b in &_tokens {
        //     print!("{}", b as char);
        // }
        // println!();

        // Write the binary data to the file
        tokens_file.write_all(&_tokens).expect("Error writing tokens");
        tokens_file.write(&['\n' as u8]).expect("Error writing tokens");

        parsed += 1;

        i += 1;
        if i % 100000 == 0 {
            let percent = (i as f64 * 100.0) / (TOTAL_GAMES as f64);
            println!("Parsed {} out of {} ({:.2}%)", parsed, i, percent);
        }

        if i > 1000000 {
            break;
        }
    }

    let total_time = total_start_time.elapsed().as_nanos() as i64;
    println!(
        "Total time taken: {}s, parse time: {}, tokenise time: {}s, parsed: {}",
        total_time as f64 / 1e9,
        total_parse_time as f64 / 1e9,
        total_tokenise_time as f64 / 1e9,
        parsed
    );
}