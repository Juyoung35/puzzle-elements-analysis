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
