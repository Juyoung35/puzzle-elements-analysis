use super::prelude::*;

pub enum ArrowShape {
  Fat { color: ArrowColor, direction: ExtendedShapeDirection },
  Thin { color: ArrowColor, direction: ExtendedShapeDirection },
  Triangular { color: ArrowColor, direction: ExtendedShapeDirection },
  FourEdge { color: ArrowColor, position: 
pub enum ArrowColor {
  Black
  Gray,
  White,
}
  