use super::prelude::*;

pub struct enum NumberShape {
  Math { color: MathColor, variant: MathVatiant },
  AcuteAngle { variant: AcuteAngleVariant },
  DigitSegment { color: DigitSegmentColor, bitflag: DigitSegmentBitflag },
  DigitalFrameSegment { bitflag: DigitSegmentBitFlag },
  DiceEyes { bitflag: DiceEyesBitflag },
  Pills { number: u8 }, // support from 1 to 5.
}

pub enum MathVariant {
  Infinity,
  Plus,
  Minus,
  Cross,
  Asterisk,
  Division,
  Equal,
  NotEqual,
  GreaterOrEqual,
  LessOrEqual,
}

pub enum AcuteAngleVariant {
  Narrow { direction: ShapeDirection },
  Wide { direction: ShapeDirection },
}

pub enum DigitSegmentColor {
  Black,
  Green
  Gray,
}

pub struct DigitalSegmentBitflag(u8);
pub const DIGITAL_SEGMENT_TOP_ID: usize = 0;
pub const DIGITAL_SEGMENT_MIDDLE_ID: usize = 1;
pub const DIGITAL_SEGMENT_BOTTOM_ID: usize = 2;
pub const DIGITAL_SEGMENT_LEFTUP_ID: usize = 3;
pub const DIGITAL_SEGMENT_LEFTDOWN_ID: usize = 4;
pub const DIGITAL_SEGMENT_RIGHTUP_ID: usize = 5;
pub const DIGITAL_SEGMENT_RIGHTDOWN_ID: usize = 6;

pub struct DiceEyesBitflag(u16);