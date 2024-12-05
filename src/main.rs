mod game {
    pub mod game;
}
mod tokenisation {
    pub mod pgnparser;
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use flate2::read::GzDecoder;
use crate::tokenisation::pgnparser;
// static mut ALL_MOVES: HashSet<Ply> = HashSet::new();
// static mut TOTAL_MOVES: i64 = 0;


static TOTAL_GAMES: i64 = 57151024;

fn main() {
    let total_start_time = Instant::now();
    let mut total_token_time = 0;
    let mut parsed = 0;
    let mut i = 0;

    // Open the gzipped file
    let file = File::open("../../python/chessai/bestgames3.gz").expect("Failed to open file");
    let reader = BufReader::new(GzDecoder::new(file));
    // let mut reader = reader.lines();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let start_time = Instant::now();
        let _game = pgnparser::parse_pgn_game(&line);
        parsed += 1;
        total_token_time += start_time.elapsed().as_nanos() as i64;

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
        "Total time taken: {}s, token time: {}s, parsed: {}",
        total_time as f64 / 1e9,
        total_token_time as f64 / 1e9,
        parsed
    );
}