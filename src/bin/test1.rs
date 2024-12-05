mod gzspeedtest;

fn main() {
    let b = '9' as u8;
    match *b {
        '0' ..'9' => {
            println!("{}", b);
        }
        _ => {}
    }
}