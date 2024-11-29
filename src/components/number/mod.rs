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

