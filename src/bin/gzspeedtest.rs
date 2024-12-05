use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use flate2::read::GzDecoder;
use std::time::Instant;

fn main() -> io::Result<()> {
    let start_time = Instant::now();
    let mut count = 0;

    let file_path = Path::new("../../../../python/chessai/bestgames3.gz");
    let file = File::open(file_path)?;
    let gz = GzDecoder::new(file);
    let reader = io::BufReader::new(gz);

    for _ in reader.lines() {
        count += 1;
        // if count == 1000000 {
        //     break;
        // }
    }

    println!("Num lines: {}", count);
    println!("Time taken: {}ms", start_time.elapsed().as_millis());

    Ok(())
}
