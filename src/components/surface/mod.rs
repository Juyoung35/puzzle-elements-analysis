use super::prelude::*;

pub struct Surface {
  cell: cell,
}

pub struct SurfaceInterface {
  color: SurfaceColor,
}

pub enum SurfaceColor {
  DarkGray,
  Gray,
  LightGray,
  Black,
  Green,
  Blue,
  Red,
  Yellow,
  Pink,
  Orange,
  Purple,
  Brown,
}
