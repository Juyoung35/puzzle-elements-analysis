use super::prelude::*;
use super::FiveColor as EdgeColor;

pub trait Edge {
}

pub struct NormalEdge {
  points: (point, point),
}

pub struct DiagonalEdge {
  points: (point, point),
}

pub struct FreeEdge {
  points: (point, point),
}

pub struct Helper {
  side: side,
}

pub struct NormalEdgeInterface {
  style: EdgeStyle,
}

pub struct NormalEdgeInterface {
  style: EdgeStyle,
}

pub struct NormalEdgeInterface {
  style: EdgeStyle,
}

pub struct HelperInterface {
}

#[derive(Default)]
pub enum EdgeStyle {
  #[default]
  Colored(EdgeColor),
  Thicker,
  Thin,
  Dotted,
  FatDots,
  Double,
}
