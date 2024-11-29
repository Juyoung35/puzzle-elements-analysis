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
