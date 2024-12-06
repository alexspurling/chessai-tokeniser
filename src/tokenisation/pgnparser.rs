use crate::game::game::{Game, Move, Winner, Ply};

pub fn parse_pgn_game(game: &str) -> Game {
    let mut moves: Vec<Move> = Vec::new();

    // Accumulator for the current number
    let mut cur_num: i32 = 0;

    // Move information
    let mut move_num: i32 = 0;
    let mut piece: char = 'x';
    let mut from_file = 'x'; // Some moves require disambiguation
    let mut from_rank = 'x'; // Some moves require disambiguation
    let mut file = 'x';
    let mut rank = 'x';
    let mut take = false;
    let mut check = false;
    let mut short_castle = false;
    let mut long_castle = false;
    let mut analysis = 0;

    // White is true if we have seen the pattern "<move_num>\. "
    let mut white = false;
    // Black is true if we have seen the pattern "<move_num>\. <white_move> "
    let mut black = false;

    // Number of O characters that we have seen
    let mut castle_os = 0;
    let mut promotion = false;
    let mut promotion_to = 'x';
    let mut checkmate = false;

    let mut winner = Winner::None;

    let mut white_ply: Option<Ply> = None;

    let chars = game.as_bytes();

    let mut i = 0;
    while i < chars.len() {
        let cur_byte = chars[i];
        let cur_char = cur_byte as char;
        if !white {
            match cur_char {
                '0'..='9' => {
                    cur_num = cur_num * 10 + (cur_char as i32 - '0' as i32);
                }
                '.' => {
                    move_num = cur_num;
                }
                _ => {}
            }
        } else {
            match cur_char {
                // Piece
                'B' | 'N' | 'Q' | 'R' | 'K' => {
                    if promotion {
                        promotion_to = cur_char;
                    } else {
                        piece = cur_char;
                    }
                }
                // File
                'a'..='h' => {
                    from_file = file;
                    file = cur_char;
                    if piece == 'x' {
                        piece = 'P';
                    }
                }
                // Rank
                '1'..='8' => {
                    from_rank = rank;
                    rank = cur_char;
                }
                // Take
                'x' => {
                    take = true;
                }
                // Check
                '+' => {
                    check = true;
                }
                // Castle
                'O' => {
                    castle_os += 1;
                }
                // Promotion
                '=' => {
                    promotion = true;
                }
                // Analysis
                '?' => {
                    let next_char = chars[i + 1] as char;
                    if next_char == '!' {
                        analysis = -1;
                        i += 1; // consume the '!'
                    } else if next_char == '?' {
                        analysis = -4;
                        i += 1; // consume the '?'
                    } else {
                        analysis = -2;
                    }
                }
                '!' => {
                    let next_char = chars[i + 1] as char;
                    if next_char == '?' {
                        analysis = 0;
                        i += 1; // consume the '?'
                    } else if next_char == '!' {
                        analysis = 4;
                        i += 1; // consume the '!'
                    } else {
                        analysis = 2;
                    }
                }
                _ => {}
            }
        }
        match cur_char {
            '#' => {
                checkmate = true;
            }
            '/' => {
                winner = Winner::Draw;
                break;
            }
            '-' => {
                if castle_os == 0 {
                    winner = if cur_num == 1 { Winner::White } else { Winner::Black };
                    break;
                }
            }
            ' ' => {
                if castle_os == 3 {
                    long_castle = true;
                } else if castle_os == 2 {
                    short_castle = true;
                }
                if white {
                    let new_ply = Ply::new(piece, from_file, from_rank, file, rank, take, check, short_castle, long_castle, promotion_to, checkmate, analysis);
                    piece = 'x';
                    from_file = 'x';
                    from_rank = 'x';
                    file = 'x';
                    rank = 'x';
                    take = false;
                    check = false;
                    short_castle = false;
                    long_castle = false;
                    promotion_to = 'x';
                    analysis = 0;
                    if black {
                        let black_ply = if new_ply.valid() { Some(new_ply) } else { None };
                        moves.push(Move::new(move_num, white_ply.unwrap(), black_ply));
                        white_ply = None;
                        white = false;
                        black = false;
                        move_num = 0;
                    } else {
                        white_ply = Some(new_ply);
                        black = true;
                    }
                } else if move_num > 0 {
                    white = true;
                }
                cur_num = 0;
                castle_os = 0;
                promotion = false;
            }
            _ => {}
        }
        i += 1;
    }

    if let Some(white_ply) = white_ply {
        moves.push(Move::new(move_num, white_ply, None));
    }

    let new_game = Game::new(moves, winner);
    // let _a = new_game.to_string();
    // let _b = game.to_string();
    // let games_match = _a == _b;
    //
    // if !games_match {
    //     println!("{}", game);
    //     println!("{}", new_game.to_string());
    // }

    return new_game
}