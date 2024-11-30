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

pub enum NormalShape {
  Circle { size: ShapeSize, variant: ShapeVariant },
  Square { size: ExtendedShapeSize, variant: ShapeVariant },
  Triangle { size: ShapeSize, variant: ShapeVariant, direction: ShapeDirection },
  Diamond { size: ShapeSize, variant: ShapeVariant },
  HexagonPoint { size: ShapeSize, variant: ShapeVariant },
  HexagonFlat { size: ShapeSize, variant: ShapeVariant },
  Primitive { color: PrimitiveColor, variant: PrimitiveVariant },
  Cross { direction: ShapeDirection },
  Lines { variant: LinesVariant },
  CageLines { variant: CageLinesVariant },
  Bars { color: BarsColor, variant: BarsVariant },
  Slope { variant: SlopeVariant },
}



pub enum ShapeSize {
  Large,
  Medium,
  Small,
  ExtraSmall,
}

pub enum ExtendedShapeSize {
  ExtraLarge,
  Large,
  Medium,
  Small,
  ExtraSmall,
}

#[derive(Default)]
pub enum ShapeVariant {
  // first prefix color is inner color.
  // second prefix color is border color.
  // trans color means it is transparent.
  // suffix describe style of border.
  #[default]
  WhiteBlack,
  BlackBlack,
  GrayTrans,
  TransBlackDotted,
  GrayBlack,
  WhiteGray,
  WhiteTrans,
  WhiteBlackThick,
  GrayBlackThick,
  WhiteBlackDouble,
}

pub enum ShapeDirection {
  East,
  North,
  West,
  South,
}

pub enum ExtendedShapeDirection {
  East,
  NorthEast,
  North,
  NorthWest,
  West,
  SouthWest,
  South,
  SouthEast,
}

pub enum DiagonalShapeDirection {
  NorthEast,
  NorthWest,
  SouthWest,
  SouthEast,
}

pub enum PrimitiveColor {
  Black,
  Green,
  Gray,
}

pub enum PrimitiveVariant {
  Circle,
  Triangle,
  Square,
  Cross,
  Diagonal { reversed: bool },
  LargeCross,
  Dot,
  CrossCircle,
}

pub enum LinesVariant {
  Horizontal,
  Vertical,
  Diagonal { reversed: bool },
  Plus,
  Cross,
  DoubleDiagonal { reversed: bool },
}

pub enum CageLinesVariant {
  GrayDottedDiagonal { reversed: bool },
  BlackDottedDiagonal { reversed: bool },
  GrayDiagonal { reversed: bool },
  BlackDiagonal { reversed: bool },
  BlackDoubleDiagonal { reversed: bool },
}

pub enum BarsColor {
  Black,
  Gray,
  White,
}

pub enum BarsVariant {
  Horizontal,
  Vertical,
  ThickHorizontal,
  ThickVertical,
}

pub enum SlopeVariant {
  Black,
  BlackSlope { filled_corner: DiagonalShapeDitection },
  Gray,
  GraySlope { filled_corner: DiagonalShapeDitection },
}