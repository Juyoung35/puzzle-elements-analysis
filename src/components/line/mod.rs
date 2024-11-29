use super::ptelude::*;
use super::FiveColor as LineColor;

pub trait Line {
}

pub struct NormalLine {
  cells: (cell, cell),
}

pub struct DiagonalLine {
  cells: (cell, cell),
}

pub struct FreeLine {
  cells: (cell, cell),
}

pub struct MiddleLine {
  cell: cell,
  side: side,
}

pub struct NormalLineInterface {
  style: LineStyle,
}

pub struct DiagonalLineInterface {
  style: LineStyle,
}

pub struct FreeLineInterface {
  style: LineStyle,
}

pub struct MiddleLineInterface {
  style: LineStyle,
}

#[derive(Default)]
pub enum LineStyle {
  #[default]
  Colored(LineColor),
  Thin,
  Dotted,
  FatDots,
  Short,
  Double,
}
