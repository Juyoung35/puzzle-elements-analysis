pub trait Number {
}

macro_rules! define_numbers {
  (ident: $ident) => {
    pub struct $ident {
      cell: cell,
      letter: String,
    }
  };
}

define_numbers!(NormalNumber, LargeNumber, MediumNumber, SmallNumber);

pub struct Candidates {
  candidates: [bool; 9],
}

pub struct CornerNumber {
  corner: corner,
  string: String,
}

pub struct PaddingNumber {
  padding: padding,
  string: String,
}
