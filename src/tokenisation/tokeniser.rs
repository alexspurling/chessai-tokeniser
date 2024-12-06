use crate::game::game::{Game, Move, Ply};

pub struct Token {
    pub token: String
}

pub fn tokenise_ply_to_str(ply: &Ply, tokens: &mut Vec<String>) {
    if ply.short_castle {
        tokens.push("O-O".to_string());
    } else if ply.long_castle {
        tokens.push("O-O-O".to_string());
    } else {
        // Piece
        tokens.push(ply.piece.into());
        if ply.take {
            tokens.push("x".to_string());
        }
        // Position
        tokens.push(format!("{}{}", ply.file, ply.rank));
    }
    if ply.check {
        tokens.push("+".to_string());
    }
}

pub fn tokenise_to_str(game: &Game) -> Vec<String> {
    let mut tokens:Vec<String> = Vec::new();

    for mv in &game.moves {
        let num = mv.num;
        tokens.push(num.to_string());
        tokenise_ply_to_str(&mv.white, &mut tokens);
        if let Some(ref black) = mv.black {
            tokenise_ply_to_str(black, &mut tokens);
        }
    }

    tokens
}

static MAX_MOVES: i32 = 149;
// Space represents 0 which shouldn't ever be used - we just use printable ascii chars to tokenise
static FIRST_NUM: u8 = ' ' as u8;
static LAST_NUM: u8 = ' ' as u8 + MAX_MOVES as u8;
static SHORT_CASTLE: u8 = LAST_NUM + 1;
static LONG_CASTLE: u8 = SHORT_CASTLE + 1;
static PAWN: u8 = LONG_CASTLE + 1;
static BISHOP: u8 = LONG_CASTLE + 2;
static KNIGHT: u8 = LONG_CASTLE + 3;
static QUEEN: u8 = LONG_CASTLE + 4;
static ROOK: u8 = LONG_CASTLE + 5;
static KING: u8 = LONG_CASTLE + 6;
static TAKE: u8 = KING + 1;
static CHECK: u8 = TAKE + 1;
static FIRST_POSITION: u8 = CHECK + 1;

pub fn tokenise_ply_to_bytes(ply: &Ply, tokens: &mut Vec<u8>) {
    if ply.short_castle {
        tokens.push(SHORT_CASTLE);
    } else if ply.long_castle {
        tokens.push(LONG_CASTLE);
    } else {
        // Piece
        let piece = match ply.piece {
            'P' => PAWN,
            'B' => BISHOP,
            'N' => KNIGHT,
            'Q' => QUEEN,
            'R' => ROOK,
            'K' => KING,
            _ => panic!("Invalid piece")
        };
        tokens.push(piece);

        if ply.take {
            tokens.push(TAKE);
        }

        let file: i32 = (ply.file as i32 - 'a' as i32);
        let rank: i32 = (ply.rank as i32 - '1' as i32);
        let position = rank * 8 + file;
        // Position
        tokens.push(FIRST_POSITION + position as u8);
    }
    if ply.check {
        tokens.push(CHECK);
    }
}

fn print_hex(vec: &Vec<u8>) {
    for byte in vec {
        print!("{:02x} ", byte); // {:02x} formats each byte as 2-digit hex
    }
    println!(); // Print a newline at the end
}

pub fn tokenise_to_bytes(game: &Game) -> Vec<u8> {
    let mut tokens:Vec<u8> = Vec::new();

    for mv in &game.moves {
        if mv.num > MAX_MOVES {
            break;
        }
        tokens.push(FIRST_NUM + mv.num as u8);

        tokenise_ply_to_bytes(&mv.white, &mut tokens);
        if let Some(ref black) = mv.black {
            tokenise_ply_to_bytes(black, &mut tokens);
        }
    }

    // print!("Token array: ");
    // print_hex(&tokens);

    tokens
}