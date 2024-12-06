

use std::fmt;

#[derive(Debug, Clone)]
pub struct Ply {
    pub piece: char,
    pub from_file: char,
    pub from_rank: char,
    pub file: char,
    pub rank: char,
    pub take: bool,
    pub check: bool,
    pub short_castle: bool,
    pub long_castle: bool,
    pub promotion_to: char,
    pub checkmate: bool,
    pub analysis: i32,
}

impl Ply {
    pub fn new(piece: char, from_file: char, from_rank: char, file: char, rank: char, take: bool, check: bool,
           short_castle: bool, long_castle: bool, promotion_to: char, checkmate: bool, analysis: i32) -> Self {
        Ply {
            piece,
            from_file,
            from_rank,
            file,
            rank,
            take,
            check,
            short_castle,
            long_castle,
            promotion_to,
            checkmate,
            analysis,
        }
    }

    pub fn valid(&self) -> bool {
        self.piece != 'x' || self.short_castle || self.long_castle
    }

    fn get_analysis_str(analysis: i32) -> &'static str {
        match analysis {
            -4 => "??",
            -2 => "?",
            -1 => "?!",
            2 => "!",
            4 => "!!",
            _ => "",
        }
    }
}


impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        if self.short_castle {
            result.push_str("O-O");
        } else if self.long_castle {
            result.push_str("O-O-O");
        } else {
            if self.piece != 'P' {
                result.push(self.piece);
            }
            if self.from_file != 'x' {
                result.push(self.from_file as char);
            }
            if self.from_rank != 'x' {
                result.push(self.from_rank as char);
            }
            if self.take {
                result.push('x');
            }
            result.push(self.file as char);
            result.push(self.rank as char);
            if self.promotion_to != 'x' {
                result.push('=');
                result.push(self.promotion_to as char);
            }
        }

        if self.check {
            result.push('+');
        }
        if self.checkmate {
            result.push('#');
        }
        if self.analysis != 0 {
            result.push_str(Ply::get_analysis_str(self.analysis));
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Winner {
    None,
    White,
    Black,
    Draw,
}

impl fmt::Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            Winner::None => "*",
            Winner::White => "1-0",
            Winner::Black => "0-1",
            Winner::Draw => "1/2-1/2",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub num: i32,
    pub white: Ply,
    pub black: Option<Ply>,
}

impl Move {
    pub fn new(num: i32, white: Ply, black: Option<Ply>) -> Self {
        Move { num, white, black }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let black_move = match &self.black {
            Some(black) if black.valid() => format!(" {}", black),
            _ => String::new(),
        };
        write!(f, "{}. {}{}", self.num, self.white, black_move)
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub moves: Vec<Move>,
    pub winner: Winner,
}

impl Game {
    pub fn new(moves: Vec<Move>, winner: Winner) -> Self {
        Game { moves, winner }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for mv in &self.moves {
            result.push_str(&mv.to_string());
            result.push_str("  ");
        }
        result.push_str(&self.winner.to_string());
        write!(f, "{}", result)
    }
}
