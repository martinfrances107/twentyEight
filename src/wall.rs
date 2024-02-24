//  A simple square with one interal block
//
//  /-¬    => TopRight   Horitonal  Top
//  |+|    => Vertical   Internal   Vertical
//  \-,    => BottomLeft Horizontal BottomRight
//
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Wall {
    TopLeft,
    Horizontal,
    TopRight,
    Vertical,
    BottomLeft,
    BottomRight,
    Internal,
}
