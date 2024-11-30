use super::prelude::*;

pub struct Shape {
  cell: cell,
  kind: ShapeKind,
}

pub enum ShapeKind {
  Normal(NormalShape),
  Number(NumberShape),
  Arrow(ArrowShape),
  Special(SpecialShape),
}