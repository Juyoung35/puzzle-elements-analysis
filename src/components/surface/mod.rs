use super::prelude::*;

pub struct Surface {
  surface: surface,
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
