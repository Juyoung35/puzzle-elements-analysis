use super::prelude::*;

pub struct Wall {
  sides: (side, side),
}

pub struct WallInterface {
  style: WallStyle,
}

#[derive(Default)]
pub enum WallStyle {
  #[default]
  Colored(FiveColor),
  Thin,
  Dotted,
  FatDots,
  GreyDots,
}
