//! A pragmatic implementation of the tic tac toe game.

use std::fmt::{Display, Formatter};

/// The field occupancy, which is either X or O if the field was played on, or Empty, if not
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FieldOcc {
    X,
    O,
    Empty,
}

pub struct Board {
    pub fields: [FieldOcc; 9],
    turn: FieldOcc,
}

impl Board {
    pub const MAX_BOARD_HASH: u16 = 19682;

    pub fn init() -> Self {
        Self {
            fields: [FieldOcc::default(); 9],
            turn: FieldOcc::STARTING_COLOR,
        }
    }

    pub fn new(fields: [FieldOcc; 9], turn: FieldOcc) -> Self {
        Self { fields, turn }
    }

    /// Hashes the board into a unique sum, ignoring whose turn it is.
    /// Note that this hash works both ways and is therefore not a real "hash"
    #[allow(dead_code)] // as this is a library function (move to lib.rs?)
    pub fn hash(&self) -> u16 {
        let mut hash = 0_u16;
        for (i, field) in self.fields.iter().enumerate() {
            hash += field.value() * 3u16.pow(i as u32);
        }
        hash
    }

    /// Parses a Board from the given two-ways hash as e.g. derived from Self#hash(&self), i. e. it
    /// takes the 9 least significant base 10 digits and parses them as base 3 digits, interpreting
    /// 0 as an empty field, 1 as an X and 2 as an O, as dictated by FieldOcc#from(u8).
    #[allow(dead_code)] // as this is a library function (todo: move to lib.rs?)
    pub fn parse(mut hash: u16, turn: FieldOcc) -> Result<Self, &'static str> {
        if hash > Self::MAX_BOARD_HASH {
            return Err("hash too big to be a valid board");
        }
        let mut fields = [FieldOcc::default(); 9];
        for i in (0..9).rev() {
            let place = 3_u16.pow(i as u32);
            let d = hash / place;
            hash -= place * d;
            fields[i] = FieldOcc::from(d);
        }
        Ok(Board::new(fields, turn))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for i in 0..3 {
            let start_index = i * 3;
            writeln!(
                f,
                "{} | {} | {}",
                self.fields[start_index],
                self.fields[start_index + 1],
                self.fields[start_index + 2]
            )?;
        }
        Ok(())
    }
}

impl Display for FieldOcc {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "X",
                Self::O => "O",
                Self::Empty => " ",
            }
        )?;
        Ok(())
    }
}

impl FieldOcc {
    pub const STARTING_COLOR: FieldOcc = Self::X;

    pub fn value(self) -> u16 {
        self as u16
    }
}

// todo: remove as derivation seems to expensive for `From`
impl From<u16> for FieldOcc {
    fn from(val: u16) -> Self {
        match val {
            0 => Self::Empty,
            1 => Self::X,
            2 => Self::O,
            _ => panic!("stfu"),
        }
    }
}

impl Default for FieldOcc {
    fn default() -> Self {
        Self::Empty
    }
}
