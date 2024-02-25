use core::fmt;

use wasm_bindgen::JsValue;
use web_sys::console::log_1;

use crate::wall::Wall;

// Cells Board infrastructure
//
// Not including charcters.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Cell {
    Dot,
    Blank,
    Empty,
    GateLeft,
    GateRight,
    PowerPill,
    W(Wall),
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'o' => Self::Dot,
            ' ' => Self::Empty,
            'P' => Self::PowerPill,
            'g' => Self::GateLeft,
            'G' => Self::GateRight,
            '+' => Self::W(Wall::Internal),
            '|' => Self::W(Wall::Vertical),
            '¬' => Self::W(Wall::TopRight),
            '/' => Self::W(Wall::TopLeft),
            '\\' => Self::W(Wall::BottomLeft),
            ',' => Self::W(Wall::BottomRight),
            '-' => Self::W(Wall::Horizontal),
            _ => {
                log_1(&JsValue::from("invalid cell conversion char"));
                log_1(&JsValue::from(value.to_string()));

                panic!("Invalid cell ")
            }
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::W(w) => {
                write!(f, "Wall{:?}", w)
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let c = Cell::Blank;
        assert_eq!(c.to_string(), "Blank")
    }
}
