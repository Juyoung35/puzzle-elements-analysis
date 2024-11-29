pub mod surface;
pub mod line;
pub mod edge;
pub mod wall;
pub mod number;
pub mod shape;
pub mod special;
pub mod cage;
pub mod box;
pub mod composite;
pub mod sudoku;

pub mod prelude {
  pub type cell = usize;
  pub type side = usize;
  pub type point = usize;
  // corner = (cell, point)
  pub type corner = usize;
  // margin = (cell, side)
  pub type margin = usize;
}

pub enum FiveColor {
  Green,
  Black,
  Gray,
  Red,
  Blue,
}
